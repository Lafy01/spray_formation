import * as React from "react";
import { cn } from "@/lib/utils"; // Adjust import path as necessary

export interface NumberInputProps
    extends React.InputHTMLAttributes<HTMLInputElement> {
    step?: number; // Allows customization of step, default to 2.5
}

const InputDay = React.forwardRef<HTMLInputElement, NumberInputProps>(
    ({ step = 0.5, className, ...props }, ref) => {
        // Ensure step is applied only for number inputs
        const validStep = step > 0 ? step : undefined;

        return (
            <input
                type="number"
                step={validStep} // Apply step attribute for number inputs
                className={cn(
                    "flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50",
                    className
                )}
                ref={ref}
                {...props}
            />
        );
    }
);

InputDay.displayName = "NumberInputStep25";

export { InputDay };
