"use client";

import React from "react";
import {
  ReviewsApi,
  User,
  WatchHistory,
  WatchHistoryApi,
  WrapperReview,
} from "@/generated";
import { Card, CardHeader } from "../../../../components/ui/card";
import { TypographyH3 } from "../../../../components/ui/typo";
import { Separator } from "@radix-ui/react-separator";
import Review from "./Review";
import { AxiosConfig } from "@/lib/utils";
import { useInfiniteQuery, useQuery } from "@tanstack/react-query";
import WatchHistoryLog from "./WatchHistoryLog";

export default function ReviewList({ user }: { user?: User | undefined }) {
  const pageSize = 2;

  const {
    data,
    isLoading,
    isError,
    fetchNextPage,
    hasNextPage,
    isFetchingNextPage,
  } = useInfiniteQuery({
    queryKey: ["watch-history", user?.id],
    queryFn: async ({ pageParam = 0 }: any) => {

      if (!user) throw new Error("User not defined");

      const api = new WatchHistoryApi(AxiosConfig);
      const res = await api.listWatchHistoryByUser(
        user?.id,
        pageSize,
        pageParam
      );
      const resData = res.data || [];

      return {
        items: resData,
        nextOffset:
          resData.length === pageSize
            ? (pageParam as number) + pageSize
            : undefined,
      };
    },
    getNextPageParam: (last: any) => last.nextOffset,
    initialPageParam: 0,
  });

  return (
    <Card>
      <CardHeader>
        <TypographyH3 str="Watch history" />
      </CardHeader>
      {isLoading ? (
        <div className="grid gap-4">
          {[...Array(5)].map((_, index) => (
            <Review key={`skeleton-${index}`} loading />
          ))}
        </div>
      ) : (
        <div className="grid gap-4 max-h-[80vh] p-2 overflow-y-auto">
          {data?.pages.flatMap(page => page.items).map((watchLog) => (
            <WatchHistoryLog key={watchLog?.id} WatchHistory={watchLog} />
          ))}
        </div>
      )}
    </Card>
  );
}
