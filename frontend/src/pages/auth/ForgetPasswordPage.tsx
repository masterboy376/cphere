import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { z } from 'zod'
import { AuthFormWrapper } from '../../wrappers/AuthFormWrapper'
import { EnvelopeIcon } from '@heroicons/react/24/outline'
import { useState } from 'react'
import authBackendApiService, { AuthResetPasswordRequest } from '../../services/auth/AuthBackendApiService'

const forgotPasswordSchema = z.object({
  email: z.string().email("Please enter a valid email address"),
})

type ForgetPasswordData = z.infer<typeof forgotPasswordSchema>

export const ForgotPasswordPage = () => {
  const { register, handleSubmit, formState: { errors } } = useForm<ForgetPasswordData>({
    resolver: zodResolver(forgotPasswordSchema)
  })

  const [loading, setLoading] = useState(false)
  const [formResponse, setFormResponse] = useState<{ status: string, message: string } | null>(null)

  const onSubmit = async (data: ForgetPasswordData) => {
    setLoading(true)
    try {
      const payload: AuthResetPasswordRequest = {
        email: data.email
      }
      const response = await authBackendApiService.resetPassword(payload)
      setFormResponse({
        "status": "success",
        "message": response?.message || "Reset link sent successfully. Please check your email."
      })
    } catch (err: any) {
      setFormResponse({
        "status": "error",
        "message": err.response?.data?.message || "Error sending reset link. Please try again."
      })
    } finally {
      setLoading(false)
    }
  }

  return (
    <AuthFormWrapper
      title="Reset Password"
      subtitle="Enter your email to reset password"
    >
      <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
        <div>
          <label className="block text-sm font-medium text-text-primary mb-2">
            Email
          </label>
          <div className="relative">
            <EnvelopeIcon className="h-5 w-5 absolute left-3 top-1/2 -translate-y-1/2 text-text-secondary" />
            <input
              {...register('email')}
              className="w-full pl-10 pr-4 py-3 bg-background rounded-lg border border-secondary/50 focus:border-primary focus:ring-2 focus:ring-primary/50"
              placeholder="email@example.com"
            />
          </div>
          {errors.email && (
            <p className="mt-1 text-sm text-red-500">{errors.email.message}</p>
          )}
        </div>

        <button
          type="submit"
          className="w-full bg-primary hover:bg-primary-dark text-white py-3 px-4 rounded-lg transition-colors"
          disabled={loading}
        >
          {loading ? "Sending..." : "Send Reset Link"}
        </button>

        {formResponse && (
          <p className={`mt-1 text-center text-sm ${formResponse.status === "success" ? "text-green-500" : "text-red-500"}`}>{formResponse.message}</p>
        )}
      </form>
    </AuthFormWrapper>
  )
}
