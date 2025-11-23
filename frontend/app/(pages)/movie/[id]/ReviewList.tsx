"use client";

import React from "react";
import { WrapperReview } from "@/generated";
import { Card, CardHeader } from "../../../../components/ui/card";
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
    <Card>
      <CardHeader>
        <TypographyH3 str="Reviews" />
      </CardHeader>
      {loading ? (
        <div className="grid gap-4">
          {[...Array(5)].map((_, index) => (
            <Review key={`skeleton-${index}`} loading />
          ))}
        </div>
      ) : (
        <div className="grid gap-4 max-h-[80vh] p-2  overflow-y-auto">
          {reviews?.map((review) => (
            <Review key={review?.id} review={review} />
          ))}
        </div>
      )}
    </Card>
  );
}
