"use client";
import { MoviesApi } from "@/generated";
import { useInfiniteQuery } from "@tanstack/react-query";
import { AxiosConfig } from "@/lib/utils";
import { TypographyH1 } from "@/components/ui/typo";
import { useDebounce } from "use-debounce";
import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import Image from "next/image";

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
      const api = new MoviesApi(AxiosConfig);
      const res = await api.searchMoviesByTitle(debouncedQ, pageSize, pageParam);
      const resData = res.data || [];

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
      <div className=" flex flex-col w-full md:max-w-6xl place-items-center">
        <TypographyH1 str="Latest Movies" />
        <Input
          value={q}
          onChange={(e) => setQ(e.target.value)}
          placeholder="Search movies by title..."
          className="my-6 w-40 md:w-60"
        />
        <div className="w-full p-6 grid grid-cols-2 md:grid-cols-3 gap-4">
          {peopleLoading ? <p>Loading movies...</p> : null}
          {peopleError ? (
            <p className="text-red-500">Failed to load movies.</p>
          ) : null}
          {data?.pages
            .flatMap((page) => page.items)
            .map((movie) => (
              <a
                key={movie.id}
                href={`/movie/${movie.id}`}
                className="block relative h-100 w-full rounded-lg overflow-hidden transform transition-transform duration-300 ease-out hover:scale-105 hover:z-10 hover:shadow-xl"
                aria-label={movie.title ?? "Movie"}
              >
                {/* Background image */}
                <Image
                  src={movie.poster_url ?? ""}
                  alt={movie.title ?? "Poster"}
                  className="absolute inset-0 w-full h-full object-cover filter brightness-75"
                  fill
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
            ))}
        </div>
        <div className="flex justify-center my-6">
          <Button
            onClick={() => fetchNextPage()}
            variant={hasNextPage ? "outline" : "secondary"}
            disabled={!hasNextPage || isFetchingNextPage}
          >
            Load More
          </Button>
        </div>
      </div>
    </main>
  );
}
