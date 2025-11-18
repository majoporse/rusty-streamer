"use client";

import React from "react";
import PersonCard from "@/components/PersonCard";
import { WrapperMovieDetail } from "@/generated";
import { Card, CardHeader } from "./ui/card";
import { TypographyH3 } from "./ui/typo";
import { Separator } from "@radix-ui/react-separator";

export default function PeopleList({
  movie,
  loading,
}: {
  movie?: WrapperMovieDetail | undefined;
  loading?: boolean;
}) {
  const people = movie?.people ?? [];
  const list = people.slice(0, 5);

  return (
    <section className="w-full h-full">
      <Card className="w-full h-full">
        <CardHeader>
          <TypographyH3 str="People" />
        </CardHeader>
        <Separator className="my-2" />
        {loading ? (
          <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-5 gap-4">
            {[...Array(5)].map((_, index) => (
              <PersonCard key={`skeleton-${index}`} loading />
            ))}
          </div>
        ) : (
          <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-5 gap-4">
            {list.map((p: any) => (
              <PersonCard
                key={p.id ?? p.name ?? p.fullName ?? Math.random()}
                person={p}
              />
            ))}
          </div>
        )}
      </Card>
    </section>
  );
}
