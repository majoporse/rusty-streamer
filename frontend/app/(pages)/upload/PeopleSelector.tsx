import { useState } from "react";
import { PersonCharacter } from "./page";
import { useDebounce } from "use-debounce";
import { useInfiniteQuery } from "@tanstack/react-query";
import { PeopleApi, WrapperPerson } from "@/generated";
import { Card } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import { Button } from "@/components/ui/button";
import PersonCard from "./PersonCard";
import { Input } from "@/components/ui/input";
import { AxiosConfig } from "@/lib/utils";

export default function PeopleSelector({
  selectedPeople,
  setSelectedPeople,
}: {
  selectedPeople: PersonCharacter[];
  setSelectedPeople: React.Dispatch<React.SetStateAction<PersonCharacter[]>>;
}) {
  const [q, setQ] = useState("");
  const [debouncedQ] = useDebounce(q, 300);

  const pageSize = 3;

  const {
    data,
    isLoading: peopleLoading,
    isError: peopleError,
    fetchNextPage,
    hasNextPage,
    isFetchingNextPage,
  } = useInfiniteQuery({
    queryKey: ["people", debouncedQ],
    queryFn: async ({ pageParam = 0 }: any) => {
      const api = new PeopleApi(AxiosConfig);
      const res = await api.getPersonByName(debouncedQ, pageSize, pageParam);
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

  function toggle(person: WrapperPerson) {
    if (selectedPeople.some((sp) => sp.id === person.id))
      setSelectedPeople((prev) => prev.filter((x) => x.id !== person.id));
    else
      setSelectedPeople((prev) => [
        ...prev,
        // ensure we store a PersonCharacter with empty character by default
        { ...person, role: "" } as PersonCharacter,
      ]);
  }

  return (
    <div className="flex flex-col">
      <label className="block text-sm font-medium mb-1">Actors</label>
      <input
        type="text"
        value={q}
        onChange={(e) => setQ(e.target.value)}
        placeholder="Search actors..."
        className="mb-2 w-full rounded-md border border-input px-3 py-2 bg-transparent"
      />

      <Card className="overflow-y-auto p-5 h-40">
        {/* people selector */}
        {peopleLoading ? (
          <Skeleton className="w-full h-full" />
        ) : peopleError ? (
          <div className="flex place-items-center text-center w-full h-full text-sm text-destructive">
            Failed to load people
          </div>
        ) : (
          <div className="grid grid-cols-2 sm:grid-cols-3 gap-2">
            {data?.pages
              .flatMap((page) => page.items)
              .map((p) => (
                <PersonCard
                  key={p.id}
                  person={p}
                  selected={selectedPeople.some((sp) => sp.id === p.id)}
                  toggle={toggle}
                />
              ))}
          </div>
        )}
      </Card>

      <div className="flex items-center gap-2 mt-2">
        <Button
          type="button"
          className="text-sm text-primary underline"
          variant="secondary"
          onClick={() => fetchNextPage?.()}
          disabled={!hasNextPage || isFetchingNextPage}
        >
          {isFetchingNextPage
            ? "Loading..."
            : hasNextPage
            ? "Load more"
            : "No more"}
        </Button>
      </div>

      <div>
        {/* selected people */}
        <label className="block text-sm font-medium mb-1">
          Selected Actors
        </label>
        <Card className="p-2 my-2 overflow-auto h-60 grid md:grid-cols-2 gap-2">
          {selectedPeople.map((person) => (
            <Card
              className="p-2 flex flex-col gap-2 h-25 flex-2"
              key={person.id}
            >
              <PersonCard
                key={person.id}
                person={person}
                selected={true}
                toggle={toggle}
              />
              {/* {person.character} */}
              <Input
                className="mt-2 w-auto"
                placeholder="Role"
                onInput={(input) => {
                  const characterPerson = selectedPeople.find(
                    (p) => p.id === person.id
                  );
                  if (!characterPerson) return;
                  characterPerson.role = input.currentTarget.value;
                  setSelectedPeople([...selectedPeople]);
                }}
              />
            </Card>
          ))}
        </Card>
      </div>
    </div>
  );
}

/* PersonCard and GenreCard moved to components/upload/PersonCard.tsx and GenreCard.tsx */
