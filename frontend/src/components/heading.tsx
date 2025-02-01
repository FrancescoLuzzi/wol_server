import { Link } from "react-router-dom";
import { useAuth } from "@/context/auth";
import {
  NavigationMenu,
  NavigationMenuItem,
  NavigationMenuLink,
  NavigationMenuList,
  navigationMenuTriggerStyle,
} from "@/components/ui/navigation-menu";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Button } from "@/components/ui/button";
//import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { Avatar } from "@/components/ui/avatar";

export function NavigationHeader() {
  const { ctx, logout } = useAuth();

  return (
    <header className="flex items-center justify-between border-b px-6 py-4">
      <Link to="/" className="flex items-center gap-2">
        <svg /* Your logo SVG */ />
        <span className="text-lg font-semibold">Your Brand</span>
      </Link>

      <NavigationMenu>
        <NavigationMenuList>
          {!ctx ? (
            <>
              <NavigationMenuItem>
                <Link to="/signup">
                  <NavigationMenuLink className={navigationMenuTriggerStyle()}>
                    Sign Up
                  </NavigationMenuLink>
                </Link>
              </NavigationMenuItem>
              <NavigationMenuItem>
                <Link to="/login">
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
                      {
                        //<AvatarImage src={ctx.avatarUrl} />
                        //<AvatarFallback>
                        //  {ctx.email?.[0]?.toUpperCase() || "U"}
                        //</AvatarFallback>
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
    </header>
  );
}
