import { Link } from "react-router-dom";
import { useAuth } from "@/hooks/auth";
import {
  NavigationMenu,
  NavigationMenuItem,
  NavigationMenuList,
  NavigationMenuTrigger,
} from "@/components/ui/navigation-menu";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Button } from "@/components/ui/button";
//import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { Avatar, AvatarFallback } from "@/components/ui/avatar";
import { ThemeToggle } from "./theme";

export function NavigationHeader() {
  const { ctx, logout } = useAuth();

  return (
    <nav className="flex items-center justify-between border-b px-6 py-4">
      <Link to="/" className="flex items-center gap-2">
        <span className="text-lg font-semibold">Your Brand</span>
      </Link>

      <NavigationMenu>
        <NavigationMenuList>
          {!ctx ? (
            <>
              <NavigationMenuItem>
                <Link to="/auth/signup">Sign Up</Link>
              </NavigationMenuItem>
              <NavigationMenuItem>
                <Link to="/auth/login">
                  <Button variant="outline">Login</Button>
                </Link>
              </NavigationMenuItem>
            </>
          ) : (
            <NavigationMenuItem>
              <DropdownMenu>
                <DropdownMenuTrigger asChild>
                  <Button
                    variant="ghost"
                    className="h-10 w-10 rounded-full p-0"
                  >
                    <Avatar className="h-8 w-8">
                      <AvatarFallback>
                        {ctx.roles[0][0]?.toUpperCase() || "U"}
                      </AvatarFallback>
                      {
                        //<AvatarImage src={ctx.avatarUrl} />
                      }
                    </Avatar>
                  </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent align="end" className="w-48">
                  <DropdownMenuItem asChild>
                    <Link to="/profile" className="cursor-pointer">
                      Profile
                    </Link>
                  </DropdownMenuItem>
                  <DropdownMenuItem
                    onClick={logout}
                    className="cursor-pointer text-destructive focus:bg-destructive/10 focus:text-destructive"
                  >
                    Logout
                  </DropdownMenuItem>
                </DropdownMenuContent>
              </DropdownMenu>
            </NavigationMenuItem>
          )}
        </NavigationMenuList>
      </NavigationMenu>
      <ThemeToggle />
    </nav>
  );
}
