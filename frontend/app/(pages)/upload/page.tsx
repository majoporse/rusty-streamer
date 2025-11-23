"use client";

import React, { useEffect, useMemo, useState } from "react";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
import { Input } from "@/components/ui/input";
import { Progress } from "@/components/ui/progress";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardHeader,
  CardContent,
  CardFooter,
} from "@/components/ui/card";

import {
  MoviesApi,
  WrapperNewMovie,
  WrapperGenre,
  WrapperPerson,
  WrapperMovieCrew,
  UploadsApi,
} from "@/generated";
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import {
  Dropzone,
  DropzoneContent,
  DropzoneEmptyState,
} from "@/components/ui/shadcn-io/dropzone";
import { Textarea } from "@/components/ui/textarea";
import GenresSelector from "./GenresSelector";
import PeopleSelector from "./PeopleSelector";
import { uploadAzureSas } from "../../../lib/azureUpload";
import { AxiosConfig } from "@/lib/utils";
import Image from "next/image";

const movieSchema = z.object({
  title: z.string().min(1, "Title is required"),
  slug: z.string().optional().or(z.literal("")),
  description: z.string().optional().or(z.literal("")),
  release_date: z.string().optional().or(z.literal("")),
  mpaa_rating: z.string().optional().or(z.literal("")),
  duration_minutes: z.number().int().positive().optional(),
});

type FormValues = z.infer<typeof movieSchema>;

export type PersonCharacter = WrapperPerson & { role: string };

export default function UploadMoviePage() {
  const form = useForm<FormValues>({
    resolver: zodResolver(movieSchema),
    defaultValues: {
      title: "",
      slug: "",
      description: "",
      release_date: "",
      duration_minutes: 90,
    },
  });

  // image preview
  const [imageFile, setImageFile] = useState<File[]>([]);
  const [movieFile, setMovieFile] = useState<File[]>([]);
  const [uploadProgress, setUploadProgress] = useState<number | null>(null);
  const [previewUrl, setFilePreview] = useState<string | null>(null);

  const [selectedPeople, setSelectedPeople] = useState<PersonCharacter[]>([]);
  const [selectedGenres, setSelectedGenres] = useState<WrapperGenre[]>([]);

  async function onSubmit(values: FormValues) {
    const wrapper: WrapperNewMovie = {
      title: values.title,
      video_url: undefined,
      poster_url: undefined,
      description: values.description,
      release_date: values.release_date,
      mpaa_rating: values.mpaa_rating,
      duration_minutes: values.duration_minutes,
      genre_ids: selectedGenres.map((g) => g.id),
      people_ids: selectedPeople.map((person, idx) => {
        return {
          person_id: person.id,
          role: person.role,
          billing_order: idx,
        } as WrapperMovieCrew;
      }),
    };

    try {
      const api = new MoviesApi(AxiosConfig);
      const azure_api = new UploadsApi(AxiosConfig);

      if (imageFile.length > 0) {
        const imgUrl = await azure_api.requestUploadSas({
          filename: imageFile[0].name,
        });
        console.log("upload url", imgUrl.data);
        setUploadProgress(0);
        await uploadAzureSas(
          imageFile[0],
          imgUrl.data.upload_url,
          (_loaded, _total, percent) => {
            setUploadProgress(percent);
          }
        );
        setUploadProgress(null);

        wrapper.poster_url = imgUrl.data.blob_url;
      }

      if (movieFile.length > 0) {
        const url = await azure_api.requestUploadSas({
          filename: movieFile[0].name,
        });

        setUploadProgress(0);
        await uploadAzureSas(
          movieFile[0],
          url.data.upload_url,
          (_loaded, _total, percent) => {
            setUploadProgress(percent);
          }
        );
        setUploadProgress(null);

        wrapper.video_url = url.data.blob_url;
      }

      const resp = await api.createMovie(wrapper);
      console.log("created", resp.data);

      form.reset();
      setSelectedPeople([]);
      setSelectedGenres([]);
      setImageFile([]);
      setMovieFile([]);
      alert(
        "Movie created (check console). If you need to upload a poster, use the posters endpoint or attach via admin."
      );
    } catch (err) {
      console.error(err);
      setUploadProgress(null);
      alert("Error creating movie — see console.");
    }
  }

  const handleDrop = (files: File[]) => {
    console.log(files);
    setMovieFile(files);

    if (files.length > 0) {
      const reader = new FileReader();
      reader.onload = (e) => {
        if (typeof e.target?.result === "string") {
          setFilePreview(e.target?.result);
        }
      };
      reader.readAsDataURL(files[0]);
    }
  };

  return (
    <main className="flex justify-center items-center w-full p-5">
      <Card className="w-full md:max-w-250">
        <CardHeader>
          <h2 className="text-xl font-semibold">Upload New Movie</h2>
        </CardHeader>

        <Form {...form}>
          <form onSubmit={form.handleSubmit(onSubmit)}>
            <CardContent>
              <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                <div className=" h-120">
                  <div className="mb-4">
                    <FormField
                      control={form.control}
                      name="title"
                      render={({ field }) => (
                        <FormItem>
                          <FormLabel>Title</FormLabel>
                          <FormControl>
                            <Input placeholder="shadcn" {...field} />
                          </FormControl>
                          <FormMessage />
                        </FormItem>
                      )}
                    />
                  </div>

                  <div className="mb-4">
                    <FormField
                      control={form.control}
                      name="slug"
                      render={({ field }) => (
                        <FormItem>
                          <FormLabel>Slug</FormLabel>
                          <FormControl>
                            <Input placeholder="optional-slug" {...field} />
                          </FormControl>
                          <FormDescription>
                            Optional URL-friendly slug.
                          </FormDescription>
                          <FormMessage />
                        </FormItem>
                      )}
                    />
                  </div>

                  <div className="mb-4">
                    <FormField
                      control={form.control}
                      name="description"
                      render={({ field }) => (
                        <FormItem>
                          <FormLabel>Description</FormLabel>
                          <FormControl>
                            <Textarea {...field} className="max-h-40 " />
                          </FormControl>
                          <FormDescription>
                            Optional description for the movie.
                          </FormDescription>
                          <FormMessage />
                        </FormItem>
                      )}
                    />
                  </div>

                  <div className="grid grid-cols-2 gap-4 mt-4">
                    <div>
                      <FormField
                        control={form.control}
                        name="release_date"
                        render={({ field }) => (
                          <FormItem>
                            <FormLabel>Release date</FormLabel>
                            <FormControl>
                              <Input type="date" {...field} />
                            </FormControl>
                            <FormMessage />
                          </FormItem>
                        )}
                      />
                    </div>

                    <div>
                      <FormField
                        control={form.control}
                        name="duration_minutes"
                        render={({ field }) => (
                          <FormItem>
                            <FormLabel>Duration (minutes)</FormLabel>
                            <FormControl>
                              <Input
                                type="number"
                                {...field}
                                // ensure valueAsNumber behavior is preserved
                                onChange={(e) =>
                                  field.onChange(Number(e.target.value))
                                }
                              />
                            </FormControl>
                            <FormMessage />
                          </FormItem>
                        )}
                      />
                    </div>
                  </div>
                </div>

                <div className="flex flex-col h-120">
                  <label className="block text-sm font-medium mb-1">
                    Poster
                  </label>
                <Dropzone
                  accept={{ "image/*": [".png", ".jpg", ".jpeg"] }}
                  maxFiles={1}
                  maxSize={1024 * 1024 * 10}
                  minSize={1024}
                  onDrop={handleDrop}
                  onError={console.error}
                  className="flex-1"
                >
                  <DropzoneEmptyState />
                    {previewUrl && (
                      <Image
                        src={previewUrl}
                        alt="preview"
                        fill
                        className="rounded-md w-full h-full object-cover"
                      />
                    )}
                </Dropzone>
                </div>
              </div>

              <div className="grid md:grid-cols-2 gap-4 mt-4 w-full">
                <div>
                  <GenresSelector
                    selectedGenres={selectedGenres}
                    setSelectedGenres={setSelectedGenres}
                  />
                  <Dropzone
                    accept={{ "video/*": [] }}
                    maxFiles={1}
                    // maxSize={1024 * 1024 * 10}
                    minSize={1024}
                    onDrop={setMovieFile}
                    onError={console.error}
                  >
                    <DropzoneEmptyState />
                    <DropzoneContent />
                  </Dropzone>
                  {uploadProgress !== null && (
                    <div className="mt-2">
                      <Progress value={uploadProgress} className="w-full" />
                      <div className="text-xs text-muted-foreground mt-1">
                        Uploading... {uploadProgress}%
                      </div>
                    </div>
                  )}
                </div>

                <PeopleSelector
                  selectedPeople={selectedPeople}
                  setSelectedPeople={setSelectedPeople}
                />
              </div>
            </CardContent>
            <CardFooter className="flex justify-end gap-2">
              <Button type="submit">Create Movie</Button>
            </CardFooter>
          </form>
        </Form>
      </Card>
    </main>
  );
}
