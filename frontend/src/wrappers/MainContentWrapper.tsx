import { ReactNode } from 'react'

import { useNavigation } from '../contexts/NavigationContext'

export const MainContentWrapper = ({
    children
}: {
    children: ReactNode
}) => {
    const { isSidebarOpen } = useNavigation()

    return (
        <main className={`flex-1 flex flex-col ${isSidebarOpen ? 'ml-2 md:ml-64' : 'ml-2'} 
        bg-background rounded-lg transition-all duration-300 ease-in-out m-2 
        p-4 z-30 min-h-0`}>

            {children}

        </main>
    )
}