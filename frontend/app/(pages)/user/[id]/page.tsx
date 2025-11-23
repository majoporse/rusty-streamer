"use client";

import { Card, CardContent, CardHeader } from "@/components/ui/card";
import { TypographyH1, TypographyH3, TypographyP } from "@/components/ui/typo";
import { ReviewsApi, User, UsersApi, WrapperReview } from "@/generated";
import { AxiosConfig } from "@/lib/utils";
import { useQuery } from "@tanstack/react-query";
import Image from "next/image";
import ReviewList from "./ReviewList";
import WatchHistoryList from "./WatchList";
import { Separator } from "@radix-ui/react-separator";
import React from "react";

type UserPageProps = {
  params: Promise<{
    id: string;
  }>;
};

export default function UserPage({ params }: UserPageProps) {
  const { id: userId } = React.use(params);

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
  let imageSrc = user?.profile_picture_url;

  return (
    <main className="w-full place-items-center flex flex-col">
      <div className="w-full md:max-w-4xl mx-auto p-6 flex flex-col gap-6">
        <TypographyH1 str="User Profile" />
        <div className="grid md:grid-cols-2 gap-6 items-start">
          {/* profile pic */}
          <div className="relative rounded-2xl overflow-hidden w-full h-150 md:h-96 bg-muted">
            {imageSrc && (
              <Image
                src={imageSrc}
                alt={user?.profile_picture_url || "User profile picture"}
                fill
                unoptimized
                className="rounded-3xl object-cover"
              />
            )}
          </div>

          {/* user bio */}
          <Card className="p-5 h-full">
            <CardHeader>
              <TypographyH3 str="Bio" />
            </CardHeader>
            <CardContent>
              {user?.bio ? (
                <p className="whitespace-pre-wrap">{user?.bio}</p>
              ) : (
                <p className="text-muted-foreground">
                  This user has not added a bio yet.
                </p>
              )}
            </CardContent>
          </Card>
        </div>

        <div className="mt-10 grid grid-cols-1 md:grid-cols-2 gap-6">
          <div>
            <ReviewList user={user} />
          </div>
          <div>
            <WatchHistoryList user={user} />
          </div>
        </div>
      </div>
    </main>
  );
}
