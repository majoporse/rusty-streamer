"use client";

import React from "react";
import { Button } from "@/components/ui/button";
import type { WrapperGenre } from "@/generated";

export default function GenreCard({
  genre,
  selected,
  toggle,
}: {
  genre: WrapperGenre;
  selected: boolean;
  toggle: (g: WrapperGenre) => void;
}) {
  return (
    <Button
      variant="outline"
      type="button"
      onClick={() => toggle(genre)}
      className={`flex items-center place-items-center transition-shadow duration-150 ${
        selected ? "border-primary ring-2 ring-primary" : "border-neutral-200"
      }`}
    >
      <div className="truncate">{genre.name || "Unknown"}</div>
    </Button>
  );
}
