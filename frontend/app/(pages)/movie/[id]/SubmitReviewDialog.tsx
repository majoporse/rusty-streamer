"use client";

import { StarRating } from "@/components/StarRating";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";
import { ReviewsApi, WrapperMovieDetail } from "@/generated";
import { zodResolver } from "@hookform/resolvers/zod";
import { useState } from "react";
import { useForm } from "react-hook-form";
import z from "zod";
import { AxiosConfig } from "@/lib/utils";

const movieSchema = z.object({
  title: z.string().min(1, "Title is required"),
  body: z.string().min(1, "Review body is required"),
});

type FormValues = z.infer<typeof movieSchema>;

export default function SubmitReviewButton({movie}: {movie: WrapperMovieDetail}) {
  const [stars, setStars] = useState(0);

  const form = useForm<FormValues>({
    resolver: zodResolver(movieSchema),
    defaultValues: {
      title: "",
      body: "",
    },
  });

  async function onSubmit(data: FormValues) {
    console.log("Submitting review:", data);
    const api = new ReviewsApi(AxiosConfig);
    const res = await api.createReview({
      title: data.title,
      body: data.body,
      movie_id: movie.id,
      rating: stars,
      user_id: "todo",
      user_name: "todo",
    });
    console.log("Review submitted:", res.data);
    form.reset();
  }

  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button variant="outline">Write a review</Button>
      </DialogTrigger>

      <DialogContent className="sm:max-w-[425px] w-full">
        <Form {...form}>
          <form onSubmit={form.handleSubmit(onSubmit)}>
            <DialogHeader>
              <DialogTitle>Submit review</DialogTitle>
              <DialogDescription>
                Make changes to your profile here. Click save when you&apos;re
                done.
              </DialogDescription>
            </DialogHeader>

            <div className="mb-4">
              <FormField
                control={form.control}
                name="title"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Title</FormLabel>
                    <FormControl>
                      <Input placeholder="shadcn" {...field} />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
            </div>

            <div className="mb-4">
              <FormField
                control={form.control}
                name="body"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Body</FormLabel>
                    <FormControl>
                      <Textarea
                        {...field}
                        className="h-40 resize-none overflow-auto"
                      />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
            </div>

            <div className="mb-4">
              <Label className="mb-2">Rating</Label>
              <StarRating value={stars} max={5} onChange={(value) => setStars(value)} />
            </div>

            <DialogFooter>
              <DialogClose asChild>
                <Button variant="outline">Cancel</Button>
              </DialogClose>
              <Button type="submit">Submit review</Button>
            </DialogFooter>
          </form>
        </Form>
      </DialogContent>
    </Dialog>
  );
}
