"use client";
import { Card } from "@/components/ui/card";
import { MoviesApi, WrapperMovie, Configuration } from "@/generated";
import { useQuery } from "@tanstack/react-query";
import { AxiosConfig } from "./layout";
import { TypographyH1 } from "@/components/ui/typo";
import { Separator } from "@radix-ui/react-separator";

export default function Home() {
  let fetch = async () => {
    const api = new MoviesApi(AxiosConfig);
    return (await api.getAllMovies(10, 0)).data;
  };

  const {
    data: movies,
    isLoading,
    isError,
  } = useQuery<WrapperMovie[]>({
    queryKey: ["movies"],
    queryFn: fetch,
  });

  return (
    <main>
      <TypographyH1 str="Latest Movies" />
      <Separator className="my-10" />

      <div className="p-6 grid grid-cols-2 md:grid-cols-4 gap-4">
        {isLoading ? <p>Loading movies...</p> : null}
        {isError ? (
          <p className="text-red-500">Failed to load movies.</p>
        ) : null}
        {movies?.map((movie) => (
          <Card key={movie.id}>
            <a href={`/movie/${movie.id}`}>
              <img src={movie.title} className="rounded-t-lg" />
              <div className="p-4">{movie.title}</div>
            </a>
          </Card>
        ))}
      </div>
    </main>
  );
}
