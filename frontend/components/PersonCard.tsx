"use client";

import React from "react";
import { Skeleton } from "@/components/ui/skeleton";
import { Card } from "./ui/card";
import { WrapperPerson } from "@/generated";

export default function PersonCard({
  person,
  loading,
}: {
  person?: WrapperPerson | undefined;
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

  const imgSrc = "/placeholder-person.png";
  const name = (person?.first_name ?? "") + (person?.last_name ?? "");
  const role = person?.role ?? "";

  return (
    <Card className="text-center">
      <img
        src={imgSrc}
        alt={name}
        className="w-full h-32 object-cover rounded-md mb-2"
      />
      <div className="text-sm font-medium">{name}</div>
      {role ? <div className="text-xs text-neutral-500">{role}</div> : null}
    </Card>
  );
}
