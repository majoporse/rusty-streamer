"use client";

import { Card, CardContent, CardHeader } from "@/components/ui/card";
import { TypographyH1 } from "@/components/ui/typo";
import { ReviewsApi, User, UsersApi, WrapperReview } from "@/generated";
import { AxiosConfig } from "@/lib/utils";
import { useQuery } from "@tanstack/react-query";
import Image from "next/image";
import ReviewList from "./ReviewList";
import WatchList from "./WatchList";

export default function UserPage() {
  const userId = "1507d84a-2d1b-414f-88e0-1201b184bd68"; // TODO: get from route params

  const fetchMovie = async () => {
    const api = new UsersApi(AxiosConfig);
    const response = await api.getUserById(userId);
    return response.data;
  };

  const {
    data: user,
    isLoading,
    isError,
  } = useQuery<User>({
    queryKey: ["user", userId],
    queryFn: fetchMovie,
  });

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
    queryKey: ["user", userId],
    queryFn: fetchReviews,
  });

  let imageSrc =
    "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcR0sMnmt0Av66zxbV0lEW2oLHGtCpo8IxZlfUX44NxPZAiykuQNH1LJvAE-FQUPGgjyf66Cy_DjZYTwoB36Lqga8j0KF1D88IDUYz2QCw&s=10";

  return (
    <main className="w-full place-items-center flex flex-col">
      <div className="w-full md:max-w-4xl mx-auto p-6 flex flex-col gap-6">
        <TypographyH1 str="User Profile" />
        <div className="grid md:grid-cols-2 gap-6 items-start">
          {/* profile pic */}
          <div className="relative rounded-2xl w-100 h-100">
            {imageSrc && (
              <Image
                src={imageSrc}
                alt={user?.display_name || "User profile picture"}
                fill
                unoptimized
                className="rounded-3xl object-cover"
              />
            )}
          </div>

          {/* user bio */}
          <Card className="p-5">
            <CardHeader>Bio</CardHeader>
            <CardContent>{user?.status}</CardContent>
          </Card>
        </div>

        <div className="mt-10 grid grid-cols-1 md:grid-cols-2 gap-6">
          <div>
            <TypographyH1 str="Reviews" />
            {/* <ReviewList reviews={reviews} loading={reviewsLoading} /> */}
          </div>
          <div>
            <TypographyH1 str="Watchlist" />
            <WatchList user={user} />
          </div>
        </div>
      </div>
    </main>
  );
}
