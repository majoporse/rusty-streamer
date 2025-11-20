import { Card } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import { GenresApi, WrapperGenre } from "@/generated";
import { useQuery } from "@tanstack/react-query";
import GenreCard from "./GenreCard";
import { Label } from "@/components/ui/label";
import { AxiosConfig } from "@/lib/utils";

export default function GenresSelector({
  selectedGenres,
  setSelectedGenres,
}: {
  selectedGenres: WrapperGenre[];
  setSelectedGenres: React.Dispatch<React.SetStateAction<WrapperGenre[]>>;
}) {
  const {
    data: genres = [],
    isLoading: genresLoading,
    isError: genresError,
  } = useQuery<WrapperGenre[]>({
    queryKey: ["genres"],
    queryFn: async () => {
      const api = new GenresApi(AxiosConfig);
      const res = await api.listGenres(100, 0);
      return res.data || [];
    },
  });

  return (
    <div className="flex flex-col w-full">
      <div>
        <label className="block text-sm font-medium mb-1">Genres</label>
        <Card className="overflow-y-auto p-5 h-40">
          {genresLoading ? (
            <Skeleton className="h-full" />
          ) : genresError ? (
            <div className="text-sm text-destructive">
              Failed to load genres
            </div>
          ) : (
            <div className="grid grid-cols-2 sm:grid-cols-3 gap-2">
              {genres?.map((g) => (
                <GenreCard
                  key={g.id}
                  genre={g}
                  selected={selectedGenres.includes(g)}
                  toggle={(genre) => {
                    if (selectedGenres.includes(genre))
                      setSelectedGenres((prev) =>
                        prev.filter((x) => x !== genre)
                      );
                    else setSelectedGenres((prev) => [...prev, genre]);
                  }}
                />
              ))}
            </div>
          )}
        </Card>
      </div>

      <div>
        <Label className="text-sm">Selected Genres</Label>
        <Card className="p-5 my-2 overflow-auto h-40 grid grid-cols-2 sm:grid-cols-3 gap-2">
          {selectedGenres.map((genre) => (
            <GenreCard
              key={genre.id}
              genre={genre}
              selected={true}
              toggle={(g) => {
                setSelectedGenres((prev) => prev.filter((x) => x !== g));
              }}
            />
          ))}
        </Card>
      </div>
    </div>
  );
}
