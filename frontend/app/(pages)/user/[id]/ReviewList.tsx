"use client";

import React from "react";
import { ReviewsApi, User, WrapperReview } from "@/generated";
import { Card, CardHeader } from "../../../../components/ui/card";
import { TypographyH3 } from "../../../../components/ui/typo";
import { Separator } from "@radix-ui/react-separator";
import Review from "./Review";
import { AxiosConfig } from "@/lib/utils";
import { useQuery } from "@tanstack/react-query";

export default function ReviewList({
  user,
}: {
  user?: User | undefined;
}) {

  const fetchReviews = async () => {
    const api = new ReviewsApi(AxiosConfig);
    const response = await api.getAllReviews();
    return response.data;
  };

  const {
    data: reviews,
    isLoading: reviewsLoading,
    isError: reviewsError,
  } = useQuery<WrapperReview[]>({
    queryKey: ["user", user?.id, "reviews"],
    queryFn: fetchReviews,
  });


  return (
    <Card>
      <CardHeader>
        <TypographyH3 str="Reviews" />
      </CardHeader>
      {reviewsLoading ? (
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
