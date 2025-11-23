import { Card } from "@/components/ui/card";
import { Rating, RatingButton } from "@/components/ui/shadcn-io/rating";
import { Skeleton } from "@/components/ui/skeleton";
import { UsersApi, WrapperReview } from "@/generated";
import { useQuery } from "@tanstack/react-query";
import { AxiosConfig } from "@/lib/utils";
import Image from "next/image";
import Example, { StarRating } from "@/components/StarRating";
import { User } from "lucide-react";

export default function Review({
  review: review,
  loading,
}: {
  review?: WrapperReview | undefined;
  loading?: boolean;
}) {
  const {
    data: person,
    isLoading,
    isError,
  } = useQuery({
    queryKey: ["person", review?.user_id],
    queryFn: async () => {
      if (!review) return null;
      const api = new UsersApi(AxiosConfig);
      const res = await api.getUserById(review.user_id);
      return res.data;
    },
  });

  const imgSrc = person?.profile_picture_url;

  if (loading) {
    return (
      <Card className="text-center p-5">
        <Skeleton className="h-4 w-3/4 mx-auto mb-1" />
        <Skeleton className="w-full h-32 rounded-md mb-2" />
        <Skeleton className="h-3 w-1/2 mx-auto" />
      </Card>
    );
  }

  return (
    <Card className="w-full p-4 rounded-md">
      <div className="flex items-start gap-4">
        <div className="flex-1">
          <div className="flex items-center justify-between gap-4">
            <div className="flex items-center gap-3">
              <div className="w-16 h-16 shrink-0 relative">
                {imgSrc ? (
                  <Image
                    src={imgSrc}
                    alt={review?.user_name || "User"}
                    className="rounded-xl object-cover"
                    fill
                  />
                ) : (
                  <User size={64} />
                )}
              </div>

              <div>
                <div className="text-sm text-muted-foreground">
                  {review?.created_at
                    ? new Date(review.created_at).toLocaleDateString()
                    : "Unknown date"}
                </div>
                <div className="text-lg font-semibold">
                  {"title " + review?.title || "Untitled review"}
                </div>
                <div className="text-sm text-neutral-500">
                  {review?.user_name || "Anonymous"}
                </div>
              </div>
            </div>

            <div className="ml-auto items-start justify-start gap-3">
              <p>{/* {review?.} */}</p>
              <Rating defaultValue={review?.rating ?? 0} readOnly>
                {Array.from({ length: 5 }).map((_, index) => (
                  <RatingButton key={index} />
                ))}
              </Rating>
            </div>
          </div>

          <Card className="mt-4 p-4 rounded-md">
            {review?.body || "No review content available."}
          </Card>
        </div>
      </div>
    </Card>
  );
}
