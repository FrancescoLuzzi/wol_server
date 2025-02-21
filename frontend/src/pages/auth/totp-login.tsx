"use client";

import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import { z } from "zod";

import { Button } from "@/components/ui/button";
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import {
  InputOTP,
  InputOTPGroup,
  InputOTPSlot,
} from "@/components/ui/input-otp";
import { useAuth } from "@/hooks/auth";
import { useCallback } from "react";
import { LoaderCircle } from "lucide-react";
import { useMutation } from "@tanstack/react-query";

const FormSchema = z.object({
  totp: z.string().length(6, {
    message: "Your one-time password must be 6 characters.",
  }),
});

export function TotpForm() {
  const { apiClient } = useAuth();
  const form = useForm<z.infer<typeof FormSchema>>({
    resolver: zodResolver(FormSchema),
    defaultValues: {
      totp: "",
    },
  });

  const validateTotp = useCallback(
    async (values: z.infer<typeof FormSchema>) => {
      await apiClient.post("/api/auth/totp", values);
    },
    [apiClient],
  );
  const { mutate, isPending } = useMutation({
    mutationFn: validateTotp,
    onSuccess: () => {
      window.location.href = "/";
    },
    onError: (error) => {
      form.setError("root", { message: error.message });
    },
  });

  return (
    <Form {...form}>
      {isPending && <LoaderCircle />}
      <form
        onSubmit={form.handleSubmit((values) => mutate(values))}
        className="m-6 w-2/3 space-y-6"
      >
        <FormField
          control={form.control}
          name="totp"
          render={({ field }) => (
            <FormItem>
              <FormLabel>One-Time Password</FormLabel>
              <FormControl>
                <InputOTP maxLength={6} {...field}>
                  <InputOTPGroup>
                    <InputOTPSlot index={0} />
                    <InputOTPSlot index={1} />
                    <InputOTPSlot index={2} />
                    <InputOTPSlot index={3} />
                    <InputOTPSlot index={4} />
                    <InputOTPSlot index={5} />
                  </InputOTPGroup>
                </InputOTP>
              </FormControl>
              <FormDescription>
                Please enter the one-time password sent to your phone.
              </FormDescription>
              <FormMessage />
            </FormItem>
          )}
        />

        <Button type="submit">Submit</Button>
      </form>
    </Form>
  );
}
