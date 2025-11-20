import { Skeleton } from "@/components/ui/skeleton";
import { WrapperMovieDetail } from "@/generated";
import Image from "next/image";

export default function MovieImage({
  movie,
  loading,
}: {
  movie?: WrapperMovieDetail | undefined;
  loading?: boolean;
}) {
  const poster = movie?.poster_url ?? "/placeholder-poster.png";

  return (
    <div className="relative w-full h-full overflow-hidden rounded-xl">
      {loading ? (
        <Skeleton className="w-full h-full rounded-xl" />
      ) : (
        <Image
          src={poster}
          alt={movie?.title ?? "Movie poster"}
          fill
          className="object-cover"
        />
      )}
    </div>
  );
}
