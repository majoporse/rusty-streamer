"use client";

import { AuthContainer } from "@/hooks/useAuth";

export default function AuthProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  return <AuthContainer.Provider>{children}</AuthContainer.Provider>;
}
