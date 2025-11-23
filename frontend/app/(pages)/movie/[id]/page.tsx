"use client";

import PeopleList from "@/app/(pages)/movie/[id]/PeopleList";
import { MoviesApi, WrapperMovieDetail } from "@/generated";
import { useQuery } from "@tanstack/react-query";
import React from "react";
import { Separator } from "@radix-ui/react-separator";
import { VideoPlayer } from "@/components/VideoPlayer";
import ReviewList from "./ReviewList";
import MovieText from "@/app/(pages)/movie/[id]/MovieText";
import MovieImage from "./MovieImage";
import { Button } from "@/components/ui/button";
import SubmitReviewButton from "./SubmitReviewDialog";
import { AxiosConfig } from "@/lib/utils";
import { AuthContainer } from "@/hooks/useAuth";

type MoviePageProps = {
  params: Promise<{
    id: string;
  }>;
};

export default function MoviePage({ params }: MoviePageProps) {
  const { id: movieId } = React.use(params);
  const auth = AuthContainer.useContainer();

  const fetchMovie = async () => {
    const api = new MoviesApi(AxiosConfig);
    const response = await api.getMovieDetailsById(movieId);
    return response.data;
  };

  const {
    data: movie,
    isLoading,
    isError,
  } = useQuery<WrapperMovieDetail>({
    queryKey: ["movie", movieId],
    queryFn: fetchMovie,
  });

  return (
    <main className="flex flex-col p-6 w-full justify-center items-center">
      <div className="w-full max-w-5xl">
        <p className="scroll-m-20 text-center text-7xl font-extrabold tracking-tight text-balance mb-5">
          {movie?.title ?? "Untitled"}
        </p>

        <div className="h-200 grid grid-cols-1 md:grid-cols-2 gap-6 my-10">
          <MovieImage movie={movie} loading={isLoading} />
          <div className="gap-5 flex flex-col">
            {/* trailer */}
            {movie && (
              <VideoPlayer
                movie={movie}
                className=""
              />
            )}

            <MovieText movie={movie} loading={isLoading} />

            <div className="flex justify-end w-full gap-4">
              <Button variant="outline">
                Add to Watchlist
              </Button>
              {auth.user ? (
                <SubmitReviewButton movie={movie} loading={isLoading}/>
              ) : (
                <Button variant="outline" disabled>
                  Log in to write a review
                </Button>
              )}
            </div>
          </div>
        </div>

        <Separator className="my-10" />

        {movie && (
          <VideoPlayer
            movie={movie}
            className="w-full rounded-xl"
          />
        )}

        <Separator className="my-10" />

        <div className="grid grid-cols-1 md:grid-cols-2 gap-6 w-full">
          <ReviewList reviews={movie?.reviews} loading={isLoading} />
          <PeopleList movie={movie} loading={isLoading} />
        </div>
      </div>
    </main>
  );
}
