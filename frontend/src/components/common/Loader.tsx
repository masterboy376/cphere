interface LoaderProps {
    message: string
}

export const Loader = ({message}: LoaderProps) => {
    return (
        <div className="flex justify-center items-center p-8">
            <div className="animate-spin rounded-full h-8 w-8 border-t-2 border-b-2 border-white"></div>
            <span className="ml-3 text-text-secondary">{message}</span>
        </div>
    )
}