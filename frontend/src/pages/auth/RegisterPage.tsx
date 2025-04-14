import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { z } from 'zod'
import { AuthFormWrapper } from '../../wrappers/AuthFormWrapper'
import { UserIcon, LockClosedIcon, EnvelopeIcon } from '@heroicons/react/24/outline'
import authBackendApiService, { AuthRegisterRequest } from '../../services/auth/AuthBackendApiService.ts'
import { useState } from 'react'
import { Link } from 'react-router-dom'

const registerSchema = z.object({
  username: z.string().min(2, "Username must be at least 2 characters").regex(/^\S*$/, "Username should not contain spaces"),
  email: z.string().email("Invalid email address"),
  password: z.string().min(8, "Password must be at least 8 characters"),
  confirmPassword: z.string()
}).refine(data => data.password === data.confirmPassword, {
  message: "Passwords don't match",
  path: ["confirmPassword"]
})

type RegisterFormData = z.infer<typeof registerSchema>

export const RegisterPage = () => {
  const { register, handleSubmit, formState: { errors } } = useForm<RegisterFormData>({
    resolver: zodResolver(registerSchema)
  })
  const [loading, setLoading] = useState(false)
  const [formResponse, setFormResponse] = useState<{ status: string, message: string } | null>(null)

  const onSubmit = async (data: RegisterFormData) => {
    setLoading(true)
    setFormResponse(null)
    try {
      const payload: AuthRegisterRequest = {
        username: data.username,
        email: data.email,
        password: data.password
      }
      const response = await authBackendApiService.register(payload)
      setFormResponse({
        "status": "success",
        "message": response?.message || "Registration successful!"
      })
      // Redirect user or show success message
    } catch (err: any) {
      setFormResponse({
        "status": "error",
        "message": err.response?.data?.message || "Registration failed!"
      })
    } finally {
      setLoading(false)
    }
  }

  return (
    <AuthFormWrapper title="Create Account" subtitle="Get started with Cphere">
      <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
        <div>
          <label className="block text-sm font-medium text-text-primary mb-2">Username</label>
          <div className="relative">
            <UserIcon className="h-5 w-5 absolute left-3 top-1/2 -translate-y-1/2 text-text-secondary" />
            <input
              {...register('username')}
              className="w-full pl-10 pr-4 py-3 bg-background rounded-lg border border-secondary/50 focus:border-primary focus:ring-2 focus:ring-primary/50"
              placeholder="example-user"
            />
          </div>
          {errors.username && <p className="mt-1 text-sm text-red-500">{errors.username.message}</p>}
        </div>

        <div>
          <label className="block text-sm font-medium text-text-primary mb-2">Email</label>
          <div className="relative">
            <EnvelopeIcon className="h-5 w-5 absolute left-3 top-1/2 -translate-y-1/2 text-text-secondary" />
            <input
              {...register('email')}
              className="w-full pl-10 pr-4 py-3 bg-background rounded-lg border border-secondary/50 focus:border-primary focus:ring-2 focus:ring-primary/50"
              placeholder="email@example.com"
            />
          </div>
          {errors.email && <p className="mt-1 text-sm text-red-500">{errors.email.message}</p>}
        </div>

        <div>
          <label className="block text-sm font-medium text-text-primary mb-2">Password</label>
          <div className="relative">
            <LockClosedIcon className="h-5 w-5 absolute left-3 top-1/2 -translate-y-1/2 text-text-secondary" />
            <input
              type="password"
              {...register('password')}
              className="w-full pl-10 pr-4 py-3 bg-background rounded-lg border border-secondary/50 focus:border-primary focus:ring-2 focus:ring-primary/50"
              placeholder="••••••••"
            />
          </div>
          {errors.password && <p className="mt-1 text-sm text-red-500">{errors.password.message}</p>}
        </div>

        <div>
          <label className="block text-sm font-medium text-text-primary mb-2">Confirm Password</label>
          <div className="relative">
            <LockClosedIcon className="h-5 w-5 absolute left-3 top-1/2 -translate-y-1/2 text-text-secondary" />
            <input
              type="password"
              {...register('confirmPassword')}
              className="w-full pl-10 pr-4 py-3 bg-background rounded-lg border border-secondary/50 focus:border-primary focus:ring-2 focus:ring-primary/50"
              placeholder="••••••••"
            />
          </div>
          {errors.confirmPassword && (
            <p className="mt-1 text-sm text-red-500">{errors.confirmPassword.message}</p>
          )}
        </div>

        <button
          type="submit"
          className="w-full bg-primary hover:bg-primary-dark text-white py-3 px-4 rounded-lg transition-colors disabled:bg-gray-400"
          disabled={loading}
        >
          {loading ? "Creating Account..." : "Create Account"}
        </button>
        {formResponse && (
          <p className={`mt-1 text-center text-sm ${formResponse.status === "success" ? "text-green-500" : "text-red-500"}`}>{formResponse.message}</p>
        )}
      </form>

      <div className="mt-6 text-center text-sm text-text-secondary">
        <Link to="/login" className="text-primary hover:text-primary-dark transition-colors">
          ← Back to Login
        </Link>
      </div>
    </AuthFormWrapper>
  )
}
