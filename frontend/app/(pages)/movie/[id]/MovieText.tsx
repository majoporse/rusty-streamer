"use client";

import React from "react";
import { Skeleton } from "@/components/ui/skeleton";
import { WrapperMovieDetail } from "@/generated";
import Example from "../../../../components/StarRating";
import { TypographyH2, TypographyP } from "../../../../components/ui/typo";
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
} from "@/components/ui/card";
import { Separator } from "@radix-ui/react-separator";



export default function MovieText({
  movie,
  loading,
}: {
  movie?: WrapperMovieDetail | undefined;
  loading?: boolean;
}) {
  return (
    <Card className="p-5 gap-0 h-full">
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
