"use client";

import {
  Box,
  ChevronUp,
  Home,
  Search,
  Upload,
  User as UserIcon,
  User2,
  Pencil,
  LogOut,
  Divide,
  LogIn,
  Clapperboard,
  Command,
} from "lucide-react";

import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@/components/ui/sidebar";

import { AuthContainer } from "@/hooks/useAuth";
import { User, UsersApi } from "@/generated";
import { AxiosConfig } from "@/lib/utils";
import { useQuery } from "@tanstack/react-query";
import Image from "next/image";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
} from "./ui/dropdown-menu";
import { DropdownMenuTrigger } from "@radix-ui/react-dropdown-menu";
import { Card } from "./ui/card";
import { useRouter } from "next/navigation";
import { NavUser } from "./navUser";
import { Avatar } from "./ui/avatar";
import { TypographyH3, TypographyH4 } from "./ui/typo";

// Menu items.
const items = [
  {
    title: "WatchRooms",
    url: "#",
    icon: Box,
  },
  {
    title: "Search",
    url: "#",
    icon: Search,
  },
];

export function AppSidebar({ ...props }: React.ComponentProps<typeof Sidebar>) {
  let router = useRouter();
  let auth = AuthContainer.useContainer();
  let userId = auth.user?._id;

  const fetchMovie = async () => {
    if (!userId) return null;
    const api = new UsersApi(AxiosConfig);
    const response = await api.getUserById(userId);
    return response.data;
  };

  const {
    data: user,
    isLoading,
    isError,
  } = useQuery<User | null>({
    queryKey: ["user", userId],
    queryFn: fetchMovie,
  });

  return (
    <Sidebar collapsible="icon" variant="floating" {...props}>
      <SidebarHeader>
        <SidebarMenu>
          <SidebarMenuItem>
            <SidebarMenuButton size="lg" asChild>
              <a href="/" className="flex items-center">
                <div className="flex aspect-square size-8 items-center justify-center rounded-lg">
                  <Clapperboard className="size-6" />
                </div>
                <TypographyH3 str="MovieApp" />
              </a>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarHeader>
      <SidebarContent>
        <SidebarGroup>
          <SidebarGroupLabel>Application</SidebarGroupLabel>
          <SidebarGroupContent>
            <SidebarMenu>
              {items.map((item) => (
                <SidebarMenuItem key={item.title}>
                  <SidebarMenuButton asChild>
                    <a href={item.url}>
                      <item.icon />
                      <span>{item.title}</span>
                    </a>
                  </SidebarMenuButton>
                </SidebarMenuItem>
              ))}
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>
      <SidebarFooter>
        {user ? (
          <NavUser user={user!} />
        ) : (
          <SidebarMenu>
            <SidebarMenuItem>
              <SidebarMenuButton asChild>
                <a href="/login">
                  <LogIn />
                  <span>Login</span>
                </a>
              </SidebarMenuButton>
            </SidebarMenuItem>
          </SidebarMenu>
        )}
      </SidebarFooter>
    </Sidebar>
  );
}
