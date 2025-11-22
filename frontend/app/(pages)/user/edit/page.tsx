"use client";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { Progress } from "@/components/ui/progress";
import {
  Dropzone,
  DropzoneContent,
  DropzoneEmptyState,
} from "@/components/ui/shadcn-io/dropzone";
import { Textarea } from "@/components/ui/textarea";
import { TypographyH3 } from "@/components/ui/typo";
import { UpdateUser, UploadsApi, UsersApi } from "@/generated";
import { uploadAzureSas } from "@/lib/azureUpload";
import { AxiosConfig } from "@/lib/utils";
import { zodResolver } from "@hookform/resolvers/zod";
import { useState } from "react";
import { useForm } from "react-hook-form";
import z from "zod";

const registerSchema = z
  .object({
    email: z.email("Enter a valid email").optional(),
    bio: z.string().optional(),
    display_name: z.string().optional(),
    password: z
      .string()
      .min(6, "Password must be at least 6 characters")
      .optional(),
    confirmPassword: z
      .string()
      .min(6, "Please confirm your password")
      .optional(),
  })
  .refine((vals) => vals.password === vals.confirmPassword, {
    message: "Passwords do not match",
    path: ["confirmPassword"],
  });

type FormValues = z.infer<typeof registerSchema>;

export default function UserEdit() {
  const [movieFile, setMovieFile] = useState<File[]>([]);
  const [uploadProgress, setUploadProgress] = useState<number | null>(null);

  const form = useForm<FormValues>({
    resolver: zodResolver(registerSchema),
  });

  async function onSubmit(data: FormValues) {
    let api = new UsersApi(AxiosConfig);
    let uploadApi = new UploadsApi(AxiosConfig);

    let upload = null;

    // upload image if exists
    if (movieFile.length > 0) {
      console.log("Uploading file:", movieFile[0]);

      upload = await uploadApi.requestUploadSas({
        filename: movieFile[0].name,
        content_type: movieFile[0].type,
      });

      let uploadResult = await uploadAzureSas(
        movieFile[0],
        upload.data.upload_url,
        (_loaded, _total, percent) => {
          setUploadProgress(percent);
        }
      );
      console.log("Upload result:", uploadResult);
      setUploadProgress(null);
    }

    api.updateUser("", {
      profile_picture_url: upload?.data.blob_url,
      email: data.email,
      password: data.password,
      display_name: data.display_name,
      bio: data.bio,
    } as UpdateUser);
  }
  const [filePreview, setFilePreview] = useState<string | undefined>();

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
    <div className="h-full w-full items-center justify-center flex flex-col p-6">
      <Card className="flex w-full max-w-sm">
        <CardHeader>
          <CardTitle>
            <TypographyH3 str={"Edit User Profile"} />
          </CardTitle>
        </CardHeader>
        <CardContent>
          <Form {...form}>
            <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
              <div>
                <Dropzone
                  accept={{ "image/*": [".png", ".jpg", ".jpeg"] }}
                  maxFiles={1}
                  maxSize={1024 * 1024 * 10}
                  minSize={1024}
                  onDrop={handleDrop}
                  onError={console.error}
                >
                  <DropzoneEmptyState />
                  {/* <DropzoneContent> */}
                  {filePreview && (
                    <div className="h-20 w-full">
                      <img
                        alt="Preview"
                        className="absolute top-0 left-0 h-full w-full object-cover"
                        src={filePreview}
                      />
                    </div>
                  )}
                  {/* </DropzoneContent> */}
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

              <FormField
                control={form.control}
                name="email"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Email</FormLabel>
                    <FormControl>
                      <Input placeholder="you@example.com" {...field} />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />

              <FormField
                control={form.control}
                name="password"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Password</FormLabel>
                    <FormControl>
                      <Input
                        type="password"
                        placeholder="••••••••"
                        {...field}
                      />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />

              <FormField
                control={form.control}
                name="confirmPassword"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Confirm password</FormLabel>
                    <FormControl>
                      <Input
                        type="password"
                        placeholder="••••••••"
                        {...field}
                      />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />

              <FormField
                control={form.control}
                name="bio"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Bio</FormLabel>
                    <FormControl>
                      <Textarea
                        placeholder="Tell us about yourself"
                        className="h-40"
                        {...field}
                      />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />

              <FormField
                control={form.control}
                name="display_name"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Display Name</FormLabel>
                    <FormControl>
                      <Input
                        type="text"
                        placeholder="Your display name"
                        {...field}
                      />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />

              <div className="flex items-center justify-between">
                <Button type="submit">Save Changes</Button>
              </div>
            </form>
          </Form>
        </CardContent>
      </Card>
    </div>
  );
}
