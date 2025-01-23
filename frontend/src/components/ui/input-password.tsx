import * as React from "react"

import { Input } from "./input"
import { CloseEye, OpenEye } from "../icon/iconApp"

export interface PasswordInputProps
    extends React.InputHTMLAttributes<HTMLInputElement> {
}

const PasswordInput = React.forwardRef<HTMLInputElement, PasswordInputProps>(
    ({ className, type, ...props }, ref) => {
        const [showPassword, setShowPassword] = React.useState(false)
        return (
            <Input className={className} 
                ref={ref}
                type={showPassword ? "text" : "password"}
                {...props}
                suffix={showPassword ? (
                    <OpenEye onClick={() => setShowPassword(false)}/>
                ) : (
                    <CloseEye onClick={() => setShowPassword(true)}/>
                )}
            />
        )
    }
)
PasswordInput.displayName = "Input"

export { PasswordInput }