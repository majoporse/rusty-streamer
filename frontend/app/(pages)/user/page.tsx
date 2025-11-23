"use client";

import { AuthContainer } from "@/hooks/useAuth";
import { useRouter } from "next/navigation";
import { useEffect } from "react";

export default function Page() {
  const auth = AuthContainer.useContainer();
  const user = auth.user;
  const router = useRouter();

  useEffect(() => {
    router.push(`/user/${user?._id}`);
  }, [router]);

  return null; // nothing to render
}
