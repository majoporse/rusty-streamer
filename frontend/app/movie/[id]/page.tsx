"use client";

import MovieDetail from "@/app/movie/[id]/MovieDetail";
import PeopleList from "@/app/movie/[id]/PeopleList";
import { MoviesApi, WrapperMovieDetail, Configuration } from "@/generated";
import { useQuery } from "@tanstack/react-query";
import React from "react";
import { AxiosConfig } from "@/app/layout";
import { TypographyH1, TypographyP } from "@/components/ui/typo";
import { Separator } from "@radix-ui/react-separator";
import { VideoPlayer } from "@/components/VideoPlayer";

type MoviePageProps = {
  params: Promise<{
    id: string;
  }>;
};

export default function MoviePage({ params }: MoviePageProps) {
  const { id: movieId } = React.use(params);

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
    <div className="flex flex-col p-6 w-full justify-center items-center">
      <p className="scroll-m-20 text-center text-6xl font-extrabold tracking-tight text-balance mb-5">
        {movie?.title ?? "Untitled"}
      </p>
      <MovieDetail movie={movie as any} loading={isLoading} />
      <Separator className="my-10" />
      {movie && (
        <VideoPlayer
          src={movie?.video_url!}
          posterSrc={movie?.poster_url ?? ""}
          className="w-full rounded-xl"
        />
      )}
    </div>
  );
}
