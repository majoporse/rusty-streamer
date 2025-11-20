
"use client";

import React from "react";
import { WrapperReview } from "@/generated";
import { Card } from "../../../../components/ui/card";
import { TypographyH3 } from "../../../../components/ui/typo";
import { Separator } from "@radix-ui/react-separator";
import Review from "./Review";

export default function ReviewList({
  reviews: reviews,
  loading,
}: {
  reviews?: WrapperReview[] | undefined;
  loading?: boolean;
}) {
  return (
    <Card className="p-5 gap-0 h-full">
      <TypographyH3 str="Reviews" />
      <Separator className="my-2" />
      {loading ? (
        <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-5 gap-4">
          {[...Array(5)].map((_, index) => (
            <Review key={`skeleton-${index}`} loading />
          ))}
        </div>
      ) : (
        <div className="grid grid-cols-2 gap-4">
          {reviews?.map((review) => (
            <Review key={review?.id} review={review} />
          ))}
        </div>
      )}
    </Card>
  );
}
