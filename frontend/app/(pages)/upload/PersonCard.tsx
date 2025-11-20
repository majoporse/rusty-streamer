"use client";

import React from "react";
import { Button } from "@/components/ui/button";
import type { WrapperPerson } from "@/generated";

export type PersonCardProps = {
  person: WrapperPerson;
  selected?: boolean;
  toggle: (person: WrapperPerson) => void;
};

export default function PersonCard({
  person,
  selected,
  toggle,
}: PersonCardProps) {
  return (
    <Button
      variant="outline"
      type="button"
      onClick={() => toggle(person)}
      className={`flex items-center gap-2 transition-shadow duration-150 ${
        selected ? "border-primary ring-2 ring-primary" : "border-neutral-200"
      }`}
    >
      <div className="truncate">
        {person.first_name + " " + person.last_name || "Unknown"}
      </div>
    </Button>
  );
}
