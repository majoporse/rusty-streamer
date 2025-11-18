"use client";

import React from "react";
import { Skeleton } from "@/components/ui/skeleton";
import { WrapperMovieDetail } from "@/generated";
import Example, { StarRating } from "./StarRating";
import { TypographyH2, TypographyP } from "./ui/typo";
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Separator } from "@radix-ui/react-separator";
import PeopleList from "./PeopleList";
import { Rating, RatingButton } from "./ui/shadcn-io/rating";
import { AspectRatio } from "@radix-ui/react-aspect-ratio";

export default function MovieDetail({
  movie,
  loading,
}: {
  movie?: WrapperMovieDetail | undefined;
  loading?: boolean;
}) {
  const poster = movie?.slug ?? "/placeholder-poster.png";

  return (
    <section
      className="
        grid gap-6 
        grid-cols-1 md:grid-cols-2
        w-full md:max-w-[1000px] 
        m-4
        h-auto md:h-200"
    >
      <div className="w-full h-auto md:h-full md:max-h-200 overflow-hidden">
        {loading ? (
          <Skeleton className="w-full h-full" />
        ) : (
          <img
            src="https://creativereview.imgix.net/uploads/2024/12/AlienRomulus-scaled.jpg?auto=compress,format&crop=faces,entropy,edges&fit=crop&q=60&w=1728&h=2560"
            alt={movie?.title ?? "Movie poster"}
            className="
          rounded-xl
          h-full w-auto
          object-cover
          mx-auto
        "
          />
        )}
      </div>

      <div
        className="
      grid gap-4
      grid-rows-2
      min-h-0 md:max-h-200
    "
      >
        <div className="min-h-0">
          <MovieTextDetail movie={movie} loading={loading} />
        </div>

        <div className="min-h-0">
          <PeopleList movie={movie} loading={loading} />
        </div>
      </div>
    </section>
  );
}

function MovieTextDetail({
  movie,
  loading,
}: {
  movie?: WrapperMovieDetail | undefined;
  loading?: boolean;
}) {
  return (
    <Card
      className="md:col-span-2"
      style={{
        height: "20rem",
        padding: "1rem",
      }}
    >
      {loading ? (
        <LoadingTextSkeleton />
      ) : (
        <>
          <CardHeader>
            <TypographyH2 str="About this movie" />
            <TypographyP
              str={movie?.description ?? "No description available."}
            />
          </CardHeader>

          <CardContent style={{ flex: 1 }}>
            <Separator className="my-4" />
          </CardContent>

          <CardFooter className="flex items-center justify-between w-full">
            <div className="flex items-center">
              {movie?.mpaa_rating == null ? (
                <span className="text-sm text-neutral-500">No ratings yet</span>
              ) : (
                <Example />
              )}
            </div>

            <div className="text-sm text-neutral-500 text-right">
              {movie?.release_date ? (
                <TypographyP str={`Released: ${movie.release_date}`} />
              ) : null}
            </div>
          </CardFooter>
        </>
      )}
    </Card>
  );
}

const LoadingTextSkeleton = () => (
  <div className="flex flex-col h-full">
    <div className="mb-3">
      <Skeleton className="h-8 w-1/3 mb-2" />
      <Skeleton className="h-4 w-full mb-2" />
      <Skeleton className="h-4 w-5/6 mb-2" />
    </div>

    <div className="flex-1">
      <Skeleton className="h-4 w-full mb-4" />
      <Skeleton className="h-4 w-4/6 mb-2" />
      <div className="flex items-center justify-between w-full mt-4">
        <Skeleton className="h-6 w-24" />
        <Skeleton className="h-4 w-32" />
      </div>
    </div>
  </div>
);
