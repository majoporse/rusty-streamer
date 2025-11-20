"use client";

import React from "react";
import { Card } from "../../../../components/ui/card";
import { TypographyH3 } from "../../../../components/ui/typo";
import { Separator } from "@radix-ui/react-separator";
import { User, WatchHistory, WatchHistoryApi } from "@/generated";
import WatchHistoryLog from "./WatchHistoryLog";
import { useQuery } from "@tanstack/react-query";
import { AxiosConfig } from "@/lib/utils";

export default function WatchList({
  user: user,
  loading,
}: {
  user?: User | undefined;
  loading?: boolean;
}) {
  const fetchWatchHistory = async () => {
    if (!user) return [];
    const api = new WatchHistoryApi(AxiosConfig);
    const response = await api.listWatchHistoryByUser(user?.id, 100, 0);
    return response.data;
  };

  const {
    data: watchLogs,
    isLoading: reviewsLoading,
    isError: reviewsError,
  } = useQuery<WatchHistory[]>({
    queryKey: [user?.id, "watchhistory"],
    queryFn: fetchWatchHistory,
  });

  return (
    <Card className="p-5 gap-0 h-full">
      <TypographyH3 str="watchhistorys" />
      <Separator className="my-2" />
      {loading ? (
        <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-5 gap-4">
          {[...Array(5)].map((_, index) => (
            <WatchHistoryLog key={`skeleton-${index}`} loading />
          ))}
        </div>
      ) : (
        <div className="grid grid-cols-2 gap-4">
          {watchLogs?.map((log) => (
            <WatchHistoryLog key={log?.id} WatchHistory={log} />
          ))}
        </div>
      )}
    </Card>
  );
}
