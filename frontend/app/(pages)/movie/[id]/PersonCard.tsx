"use client";

import React from "react";
import { Skeleton } from "@/components/ui/skeleton";
import { Card } from "@/components/ui/card";
import { MovieCrewDetail } from "@/generated";
import { TypographyH4 } from "@/components/ui/typo";
import Image from "next/image";

export default function PersonCard({
  person: crew,
  loading,
}: {
  person?: MovieCrewDetail | undefined;
  loading?: boolean;
}) {
  if (loading) {
    return (
      <Card className="text-center">
        <Skeleton className="w-full h-32 rounded-md mb-2" />
        <Skeleton className="h-4 w-3/4 mx-auto mb-1" />
        <Skeleton className="h-3 w-1/2 mx-auto" />
      </Card>
    );
  }
  const imgSrc =
    crew?.person.image_url ??
    "https://hips.hearstapps.com/hmg-prod/images/christopher-nolan-attends-the-oppenheimer-premiere-at-news-photo-1704643272.jpg?crop=0.669xw:1.00xh;0.187xw,0&resize=640:*";
  const role = crew?.character_name ?? "";

  return (
    <Card className="relative text-left w-full p-0 overflow-hidden rounded-md">
      <div className="relative w-full h-44">
        <Image
          src={imgSrc}
          alt={crew?.person.first_name || "Person Image"}
          fill
          className="object-cover"
        />

        <div
          className="absolute inset-0"
          style={{
            background:
              "linear-gradient(to top, rgba(0,0,0,0.85), rgba(0,0,0,0.50), rgba(0,0,0,0.12))",
          }}
        />

        <div className="absolute bottom-3 left-3 right-3 text-white z-10">
          <div className="text-xs text-white/80">{role}</div>
          <div className="flex flex-col gap-0">
            <TypographyH4 str={crew?.person.first_name || "Unknown"} />
            <TypographyH4 str={crew?.person.last_name || "Unknown"} />
          </div>
        </div>
      </div>
    </Card>
  );
}
