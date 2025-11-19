"use client";
import { Card } from "@/components/ui/card";
import { MoviesApi, WrapperMovie, Configuration } from "@/generated";
import { useInfiniteQuery, useQuery } from "@tanstack/react-query";
import { AxiosConfig } from "./layout";
import { TypographyH1 } from "@/components/ui/typo";
import { Separator } from "@radix-ui/react-separator";
import { useDebounce } from "use-debounce";
import { useState } from "react";

export default function Home() {
  const [q, setQ] = useState("");
  const [debouncedQ] = useDebounce(q, 300);
  const pageSize = 2;

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
      let api = new MoviesApi(AxiosConfig);
      let res = await api.searchMoviesByTitle(debouncedQ, pageSize, pageParam);
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


  return (
    <main className="flex flex-col items-center p-6 w-full">
      <div className="w-full md:max-w-5xl">
        <TypographyH1 str="Latest Movies" />
        <Separator className="my-10" />

        <div className="p-6 grid grid-cols-2 md:grid-cols-3 gap-4">
          {peopleLoading ? <p>Loading movies...</p> : null}
          {peopleError ? (
            <p className="text-red-500">Failed to load movies.</p>
          ) : null}
          {data?.pages.flatMap(page => page.items).map((movie) =>
            <a
              key={movie.id}
              href={`/movie/${movie.id}`}
              className="block relative h-100 w-full rounded-lg overflow-hidden transform transition-transform duration-300 ease-out hover:scale-105 hover:z-10 hover:shadow-xl"
              aria-label={movie.title ?? "Movie"}
            >
              {/* Background image */}
              <img
                src={movie.poster_url ?? ""}
                alt={movie.title ?? "Poster"}
                className="absolute inset-0 w-full h-full object-cover filter brightness-75"
              />

              {/* Faded gradient overlay to make text readable */}
              <div className="absolute inset-0 bg-linear-to-t from-black/70 via-black/10 to-transparent" />

              {/* Text overlay */}
              <div className="relative z-10 h-full flex items-end p-4">
                <h3 className="text-white text-lg font-semibold drop-shadow">
                  {movie.title}
                </h3>
              </div>
            </a>
          )}
        </div>
      </div>
    </main>
  );
}
