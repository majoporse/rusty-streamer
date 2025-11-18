"use client";

import VideoPlayer from "@/components/VideoPlayer";
import MovieDetail from "@/components/MovieDetail";
import PeopleList from "@/components/PeopleList";
import { MoviesApi, WrapperMovieDetail, Configuration } from "@/generated";
import { useQuery } from "@tanstack/react-query";
import React from "react";
import { AxiosConfig } from "@/app/layout";
import { TypographyH1, TypographyP } from "@/components/ui/typo";
import { Separator } from "@radix-ui/react-separator";
import { Skeleton } from "@/components/ui/skeleton";

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
      <TypographyH1 str={movie?.title ?? "Untitled"} />
      <Separator className="my-10" />
      <MovieDetail movie={movie as any} loading={isLoading} />
      <Separator className="my-10" />
      <VideoPlayer src="https://d2zihajmogu5jn.cloudfront.net/bipbop-advanced/bipbop_16x9_variant.m3u8" />
    </div>
  );
}
