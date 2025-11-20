"use client";

import React from "react";
import { WrapperMovieDetail } from "@/generated";
import { Card } from "../../../../components/ui/card";
import { TypographyH3 } from "../../../../components/ui/typo";
import { Separator } from "@radix-ui/react-separator";
import PersonCard from "./PersonCard";

export default function PeopleList({
  movie,
  loading,
}: {
  movie?: WrapperMovieDetail | undefined;
  loading?: boolean;
}) {
  const people = movie?.people ?? [];

  return (
    <Card className="p-5 gap-0 h-full">
      <TypographyH3 str="People" />
      <Separator className="my-2" />
      {loading ? (
        <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-5 gap-4">
          {[...Array(5)].map((_, index) => (
            <PersonCard key={`skeleton-${index}`} loading />
          ))}
        </div>
      ) : (
        <div className="grid grid-cols-2 gap-4">
          {people.map((p) => (
            <PersonCard key={p.person.id} person={p} />
          ))}
        </div>
      )}
    </Card>
  );
}
