"use client";

import React, { useState } from "react";
import { Controller, useForm } from "react-hook-form";
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
import { useQuery, useInfiniteQuery } from "@tanstack/react-query";
import { useDebounce } from "use-debounce";
// using simple labels instead of shadcn Label here
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardHeader,
  CardContent,
  CardFooter,
} from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import { AspectRatio } from "@/components/ui/aspect-ratio";

import {
  MoviesApi,
  PeopleApi,
  WrapperNewMovie,
  WrapperGenre,
  WrapperPerson,
  WrapperMovieCrew,
} from "@/generated";
import { AxiosConfig } from "@/app/layout";
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
import { Label } from "@radix-ui/react-dropdown-menu";

const movieSchema = z.object({
  title: z.string().min(1, "Title is required"),
  slug: z.string().optional().or(z.literal("")),
  description: z.string().optional().or(z.literal("")),
  release_date: z.string().optional().or(z.literal("")),
  mpaa_rating: z.string().optional().or(z.literal("")),
  duration_minutes: z.number().int().positive().optional(),
});

type FormValues = z.infer<typeof movieSchema> & {
  // we'll track selected genre ids separately
};

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
  const [previewUrl, setPreviewUrl] = useState<string | null>(null);

  // create object URL for preview when files change
  React.useEffect(() => {
    if (imageFile && imageFile.length > 0) {
      const file = imageFile[0];
      const url = URL.createObjectURL(file);
      setPreviewUrl(url);
      return () => {
        URL.revokeObjectURL(url);
      };
    }

    setPreviewUrl(null);
    return;
  }, [imageFile]);

  const [selectedPeople, setSelectedPeople] = useState<WrapperPerson[]>([]);
  const [selectedGenres, setSelectedGenres] = useState<WrapperGenre[]>([]);

  async function onSubmit(values: FormValues) {
    // Build WrapperNewMovie
    const wrapper: WrapperNewMovie = {
      title: values.title,
      // coerce optional fields to strings or undefined as the API expects
      slug: values.slug ?? "",
      description: values.description ?? "",
      release_date: values.release_date ?? "",
      mpaa_rating: values.mpaa_rating ?? "",
      duration_minutes: values.duration_minutes ?? undefined,
      genre_ids: selectedGenres.length ? selectedGenres : undefined,
      people_ids: selectedPeople.map(
        (pid, idx) => ({ person_id: pid, billing_order: idx + 1 } as any)
      ),
    };

    try {
      const api = new MoviesApi(AxiosConfig);
      const resp = await api.createMovie(wrapper);
      console.log("created", resp.data);
      // reset form
      form.reset();
      setSelectedPeople([]);
      setImageFile([]);
      setPreviewUrl(null);
      alert(
        "Movie created (check console). If you need to upload a poster, use the posters endpoint or attach via admin."
      );
    } catch (err) {
      console.error(err);
      alert("Error creating movie — see console.");
    }
  }

  return (
    <main className="p-6">
      <Card>
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
                            <textarea
                              {...field}
                              className="w-full min-h-32 rounded-md border border-input px-3 py-2 bg-transparent"
                            />
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
                    accept={{ "image/*": [] }}
                    maxFiles={10}
                    maxSize={1024 * 1024 * 10}
                    minSize={1024}
                    onDrop={setImageFile}
                    onError={console.error}
                  >
                    <DropzoneEmptyState />
                    <DropzoneContent />
                  </Dropzone>

                  <div className="w-full h-full my-2 overflow-hidden">
                    {previewUrl ? (
                      <img
                        src={previewUrl}
                        alt="preview"
                        className="rounded-md w-full h-full object-cover"
                      />
                    ) : imageFile.length > 0 ? (
                      <div className="border border-dashed border-neutral-200 p-6 text-center text-sm text-neutral-500 rounded-xl">
                        {imageFile[0].name}
                      </div>
                    ) : (
                      <div className="flex h-full border border-dashed border-neutral-200 p-6 text-center text-sm text-neutral-500 rounded-xl">
                        No image selected
                      </div>
                    )}
                  </div>
                </div>
              </div>

              <div className="grid md:grid-cols-2 gap-4 mt-4 items-stretch w-full">
                <GenresSelector
                  selectedGenres={selectedGenres}
                  setSelectedGenres={setSelectedGenres}
                />
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

function GenresSelector({
  selectedGenres,
  setSelectedGenres,
}: {
  selectedGenres: WrapperGenre[];
  setSelectedGenres: React.Dispatch<React.SetStateAction<WrapperGenre[]>>;
}) {
  const {
    data: genres = [],
    isLoading: genresLoading,
    isError: genresError,
  } = useQuery<WrapperGenre[]>({
    queryKey: ["genres"],
    queryFn: async () => {
      let api = new MoviesApi(AxiosConfig);
      return [];
    },
  });

  return (
    <div className="flex flex-col w-full">
      <div>
        <label className="block text-sm font-medium mb-1">Genres</label>
        <Card className="overflow-y-auto p-5 h-40">
          {genresLoading ? (
            <Skeleton className="h-full" />
          ) : genresError ? (
            <div className="text-sm text-destructive">
              Failed to load genres
            </div>
          ) : (
            <div className="grid grid-cols-2 sm:grid-cols-3 gap-2">
              {genres?.map((g) => (
                <GenreCard
                  key={g.id}
                  genre={g}
                  selected={selectedGenres.includes(g)}
                  toggle={(genre) => {
                    if (selectedGenres.includes(genre))
                      setSelectedGenres((prev) =>
                        prev.filter((x) => x !== genre)
                      );
                    else setSelectedGenres((prev) => [...prev, genre]);
                  }}
                />
              ))}
            </div>
          )}
        </Card>
      </div>

      <div>
        <Label className="text-sm">Selected Genres</Label>
        <Card className="overflow-y-auto p-5 h-40">
          {selectedGenres.map((genre) => (
            <GenreCard
              key={genre.id}
              genre={genre}
              selected={true}
              toggle={(g) => {
                setSelectedGenres((prev) => prev.filter((x) => x !== g));
              }}
            />
          ))}
        </Card>
      </div>
    </div>
  );
}

function PeopleSelector({
  selectedPeople,
  setSelectedPeople,
}: {
  selectedPeople: WrapperPerson[];
  setSelectedPeople: React.Dispatch<React.SetStateAction<WrapperPerson[]>>;
}) {
  const [q, setQ] = React.useState("");
  const [debouncedQ] = useDebounce(q, 300);

  const pageSize = 3;

  const {
    data,
    isLoading: peopleLoading,
    isError: peopleError,
    fetchNextPage,
    hasNextPage,
    isFetchingNextPage,
  } = useInfiniteQuery({
    queryKey: ["people", debouncedQ],
    queryFn: async ({ pageParam = 0 }: any) => {
      let api = new PeopleApi(AxiosConfig);
      let res = await api.getPersonByName(debouncedQ, pageSize, pageParam);
      let resData = res.data || [];

      return {
        items: resData,
        nextOffset:
          resData.length === pageSize
            ? (pageParam as number) + pageSize
            : undefined,
      };
    },
    getNextPageParam: (last: any) => last.nextOffset,
    initialPageParam: 0,
  });

  function toggle(person: WrapperPerson) {
    if (selectedPeople.includes(person))
      setSelectedPeople((prev) => prev.filter((x) => x !== person));
    else setSelectedPeople((prev) => [...prev, person]);
  }

  return (
    <div className="flex flex-col">
      <label className="block text-sm font-medium mb-1">Actors</label>
      <input
        type="text"
        value={q}
        onChange={(e) => setQ(e.target.value)}
        placeholder="Search actors..."
        className="mb-2 w-full rounded-md border border-input px-3 py-2 bg-transparent"
      />

      <Card className="overflow-y-auto p-5 h-40">
        {/* people selector */}
        {peopleLoading ? (
          <Skeleton className="w-full h-full" />
        ) : peopleError ? (
          <div className="flex place-items-center text-center w-full h-full text-sm text-destructive">
            Failed to load people
          </div>
        ) : (
          <div className="grid grid-cols-2 sm:grid-cols-3 gap-2">
            {data?.pages
              .flatMap((page) => page.items)
              .map((p) => (
                <PersonCard
                  key={p.id}
                  person={p}
                  selected={selectedPeople.includes(p)}
                  toggle={toggle}
                />
              ))}
          </div>
        )}
      </Card>

      <div className="flex items-center gap-2 mt-2">
        <Button
          type="button"
          className="text-sm text-primary underline"
          variant="secondary"
          onClick={() => fetchNextPage?.()}
          disabled={!hasNextPage || isFetchingNextPage}
        >
          {isFetchingNextPage
            ? "Loading..."
            : hasNextPage
            ? "Load more"
            : "No more"}
        </Button>
      </div>

      <div>
        {/* selected people */}
        <label className="block text-sm font-medium mb-1">
          Selected Actors
        </label>
        <Card className="p-5 my-2 overflow-auto h-40 grid grid-cols-2 sm:grid-cols-3 gap-2">
          {selectedPeople.map((person) => (
            <PersonCard
              key={person.id}
              person={person}
              selected={true}
              toggle={toggle}
            />
          ))}
        </Card>
      </div>
    </div>
  );
}

const PersonCard = ({
  person,
  selected,
  toggle,
}: {
  person: WrapperPerson;
  selected: boolean;
  toggle: (person: WrapperPerson) => void;
}) => (
  <Button
    variant="outline"
    key={person.id}
    type="button"
    onClick={() => toggle(person)}
    className={`flex place-items-center transition-shadow duration-150 ${
      selected ? "border-primary ring-2 ring-primary" : "border-neutral-200"
    }`}
  >
    {/* <div className="flex-0 w-10 h-10 bg-neutral-100 rounded-md overflow-hidden">
      {person.imageUrl ? (
        <img
          src={(p as any).photo_url ?? (p as any).image}
          alt={person.first_name}
          className="w-full h-full object-cover"
        />
      ) : (
      <div className="w-full h-full bg-neutral-200" />
      )}
    </div> */}
    <div className="truncate">
      {person.first_name + " " + person.last_name || "Unknown"}
    </div>
  </Button>
);

const GenreCard = ({
  genre,
  selected,
  toggle,
}: {
  genre: WrapperGenre;
  selected: boolean;
  toggle: (genre: WrapperGenre) => void;
}) => (
  <Button
    variant="outline"
    key={genre.id}
    type="button"
    onClick={() => toggle(genre)}
    className={`flex place-items-center transition-shadow duration-150 ${
      selected ? "border-primary ring-2 ring-primary" : "border-neutral-200"
    }`}
  >
    <div className="truncate">{genre.name || "Unknown"}</div>
  </Button>
);
