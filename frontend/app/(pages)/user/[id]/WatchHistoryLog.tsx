import StarRating from "@/components/StarRating";
import { Card } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import { WatchHistory } from "@/generated";
import { Stars } from "lucide-react";

type Props = {
  WatchHistory?: WatchHistory;
  loading?: boolean;
};

export default function WatchHistoryLog(props: Props) {
  return (
    <div>
      <Card className="p-5 gap-0 h-full">
        <div className="flex flex-col items-center justify-center">
          {props.loading ? (
            <Skeleton className="w-full h-48 mb-4" />
          ) : (
            <>
              <p className="text-lg font-medium">
                {props.WatchHistory?.content_id || "Untitled Movie"}
              </p>
              <p className="text-sm text-neutral-500">Watched on: </p>
            </>
          )}

          {props.loading ? (
            <Skeleton className="w-24 h-6 mt-2" />
          ) : (
            <p className="text-sm text-neutral-500">
              {props.WatchHistory?.last_watched_at
                ? new Date(props.WatchHistory.last_watched_at).toLocaleDateString()
                : "Unknown Date"}
            </p>
          )}

          {props.loading ? (
            <Skeleton className="w-16 h-6 mt-2" />
          ) : (
            <p className="text-sm text-neutral-500">
              Duration: {props.WatchHistory?.progress_seconds || 0} seconds
            </p>
          )}

        </div>
      </Card>
    </div>
  );
}
