"use client";

import { useState } from "react";
import { cn } from "@/lib/utils";
import { Star } from "lucide-react";

interface StarRatingProps {
  value?: number;
  max?: number;
  size?: number;
  onChange?: (value: number) => void;
}

export function StarRating({
  value = 0,
  max = 5,
  size = 24,
  onChange,
}: StarRatingProps) {
  const [hover, setHover] = useState<number | null>(null);

  return (
    <div className="flex gap-1">
      {Array.from({ length: max }, (_, i) => i + 1).map((star) => (
        <button
          key={star}
          onMouseEnter={() => setHover(star)}
          onMouseLeave={() => setHover(null)}
          onClick={() => onChange?.(star)}
          type="button"
          className="p-0 m-0 bg-transparent border-none cursor-pointer"
        >
          <Star
            size={size}
            className={cn(
              "transition-colors",
              (hover ?? value) >= star
                ? "fill-yellow-400 text-yellow-400"
                : "text-gray-300"
            )}
          />
        </button>
      ))}
    </div>
  );
}

import { Rating, RatingButton } from '@/components/ui/shadcn-io/rating';

const Example = () => (
  <Rating defaultValue={3}>
    {Array.from({ length: 5 }).map((_, index) => (
      <RatingButton key={index} />
    ))}
  </Rating>
);

export default Example;