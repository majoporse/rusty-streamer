import StarRating from "@/components/StarRating";
import { Card } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import { WatchHistory } from "@/generated";
import { Separator } from "@radix-ui/react-separator";
import { Stars } from "lucide-react";

type Props = {
  WatchHistory?: WatchHistory;
  loading?: boolean;
};

export default function WatchHistoryLog(props: Props) {
  return (
    <div>
      <Card className="p-5 gap-0 h-full">
        <div className="gap-1 flex flex-col items-center justify-center">
          {props.loading ? (
            <Skeleton className="w-full h-48 mb-4" />
          ) : (
            <p className="text-lg font-medium">
              {props.WatchHistory?.content_id || "Untitled Movie"}
            </p>
          )}

          {props.loading ? (
            <Skeleton className="w-16 h-6 mt-2" />
          ) : (
            <p className="text-sm text-neutral-500">
              Watched:
              {" " +
                formatSecsondsToHMS(props.WatchHistory?.progress_seconds || 0)}
            </p>
          )}

          {props.loading ? (
            <Skeleton className="w-24 h-6 mt-2" />
          ) : (
            <>
              <p className="text-sm text-neutral-500">Watched on: </p>
              <p className="text-sm text-neutral-500">
                {props.WatchHistory?.last_watched_at
                  ? new Date(
                      props.WatchHistory.last_watched_at
                    ).toLocaleDateString()
                  : "Unknown Date"}
              </p>
            </>
          )}
        </div>
      </Card>
    </div>
  );
}

function formatSecsondsToHMS(seconds: number): string {
  const h = Math.floor(seconds / 3600)
    .toString()
    .padStart(2, "0");
  const m = Math.floor((seconds % 3600) / 60)
    .toString()
    .padStart(2, "0");
  const s = Math.floor(seconds % 60)
    .toString()
    .padStart(2, "0");
  return `${h}:${m}:${s}`;
}
