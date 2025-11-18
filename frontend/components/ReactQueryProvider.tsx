"use client";

import React from "react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

// Create a single QueryClient instance for the app.
const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      // retry failed requests twice
      retry: 2,
      // keep data fresh for 5 minutes
      staleTime: 1000 * 60 * 5,
      // refetch on window focus can be enabled/disabled here
      refetchOnWindowFocus: false,
    },
  },
});

export default function ReactQueryProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
  );
}
