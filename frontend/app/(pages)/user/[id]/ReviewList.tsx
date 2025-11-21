
"use client";

import React from "react";
import { ReviewsApi, User, WrapperReview } from "@/generated";
import { Card } from "../../../../components/ui/card";
import { TypographyH3 } from "../../../../components/ui/typo";
import { Separator } from "@radix-ui/react-separator";
import Review from "./Review";
import { AxiosConfig } from "@/lib/utils";
import { useQuery } from "@tanstack/react-query";

export default function ReviewList({
  user,
  loading,
}: {
  user?: User | undefined;
  loading?: boolean;
}) {

  const fetchReviews = async () => {
    if (!user) return [];
    const api = new ReviewsApi(AxiosConfig);
    const response = await api.getReviewsByUserId(user?.id, 100, 0);
    return response.data;
  };

  const {
    data: reviews,
    isLoading: reviewsLoading,
    isError: reviewsError,
  } = useQuery<WrapperReview[]>({
    queryKey: [user?.id, "reviews"],
    queryFn: fetchReviews,
  });

  return (
    <Card className="p-5 gap-0 h-full">
      <Separator className="my-2" />
      {loading ? (
        <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-5 gap-4">
          {[...Array(5)].map((_, index) => (
            <Review key={`skeleton-${index}`} loading  user={user} />
          ))}
        </div>
      ) :   (
        <div className="grid grid-cols-2 gap-4">
          {reviews?.map((review) => (
            <Review key={review?.id} review={review} user={user} />
          ))}
        </div>
      )}
    </Card>
  );
}
