import { Card } from "@/components/ui/card";
import { Rating } from "@/components/ui/shadcn-io/rating";
import { Skeleton } from "@/components/ui/skeleton";
import { UsersApi, WrapperReview } from "@/generated";
import { useQuery } from "@tanstack/react-query";
import { AxiosConfig } from "@/lib/utils";
import Image from "next/image";

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

  const imgSrc =
    person?.profile_picture_url ??
    "https://hips.hearstapps.com/hmg-prod/images/christopher-nolan-attends-the-oppenheimer-premiere-at-news-photo-1704643272.jpg?crop=0.669xw:1.00xh;0.187xw,0&resize=640:*";
  if (loading) {
    return (
      <Card className="text-center">
        <Skeleton className="w-full h-32 rounded-md mb-2" />
        <Skeleton className="h-4 w-3/4 mx-auto mb-1" />
        <Skeleton className="h-3 w-1/2 mx-auto" />
      </Card>
    );
  }
  return (
    <Card className="w-full p-4 rounded-md">
      <div className="flex items-start gap-4">
        <div className="flex-1">
          <div className="flex items-center justify-between gap-4">
            <div>
              <div className="text-sm text-muted-foreground">
                {review?.created_at
                  ? new Date(review.created_at).toLocaleDateString()
                  : "Unknown date"}
              </div>
              <div className="text-lg font-semibold">
                {review?.title || "Untitled review"}
              </div>
            </div>
            <div className="text-sm text-neutral-500">
              {review?.user_name || "Anonymous"}
            </div>
          </div>

          <div className="mt-2 flex items-center gap-3">
            <Rating value={review?.rating ?? 0} readOnly />
          </div>

          {review?.body && (
            <div className="mt-3 text-sm text-neutral-700">{review.body}</div>
          )}
        </div>

        <div className="w-16 h-16 shrink-0 relative">
          <Image
            src={imgSrc}
            alt={review?.user_name || "User"}
            className="rounded-full object-cover border border-neutral-200"
            fill
          />
        </div>
      </div>
    </Card>
  );
}
