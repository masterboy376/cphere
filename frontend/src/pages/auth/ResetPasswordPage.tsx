import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { z } from 'zod'
import { useParams, useNavigate } from 'react-router-dom'
import { AuthFormWrapper } from '../../wrappers/AuthFormWrapper'
import { LockClosedIcon } from '@heroicons/react/24/outline'
import authBackendApiService, { AuthChangePasswordRequest } from '../../services/auth/AuthBackendApiService'
import { useState } from 'react'

const resetPasswordSchema = z
  .object({
    password: z.string().min(8, 'Password must be at least 8 characters'),
    confirmPassword: z.string(),
  })
  .refine((data) => data.password === data.confirmPassword, {
    message: "Passwords don't match",
    path: ['confirmPassword'],
  })

type ResetPasswordData = z.infer<typeof resetPasswordSchema>

export const ResetPasswordPage = () => {
  const { token } = useParams()
  const navigate = useNavigate()
  const [loading, setLoading] = useState(false)
  const [formResponse, setFormResponse] = useState<{ status: string, message: string } | null>(null)

  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<ResetPasswordData>({
    resolver: zodResolver(resetPasswordSchema),
  })

  const onSubmit = async (data: ResetPasswordData) => {
    setLoading(true)
    setFormResponse(null)
    if (!token) {
      setFormResponse({
        "status": "error",
        "message": "Invalid reset link!"
      })
      return
    }

    try {
      let payload: AuthChangePasswordRequest = {
        reset_token: token,
        new_password: data.password,
      }
      await authBackendApiService.changePassword(payload)
      setFormResponse({
        "status": "success",
        "message": "Password reset successfully! Redirecting..."
      })
      setTimeout(() => navigate('/login'), 3000)
    } catch (err: any) {
      setFormResponse({
        "status": "error",
        "message": err.response?.data?.message || "Failed to send reset link!"
      })
    } finally {
      setLoading(false)
    }
  }

  return (
    <AuthFormWrapper title="Set New Password" subtitle="Enter your new password below">
      <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">

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
          className="w-full bg-primary hover:bg-primary-dark text-white py-3 px-4 rounded-lg transition-colors"
          disabled={loading}
        >
          {loading ? "Changing password..." : "Reset Password"}
        </button>
        {formResponse && (
          <p className={`mt-1 text-center text-sm ${formResponse.status === "success" ? "text-green-500" : "text-red-500"}`}>{formResponse.message}</p>
        )}
      </form>
    </AuthFormWrapper>
  )
}
