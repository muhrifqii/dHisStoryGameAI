import * as React from 'react'
import botImg from '/bot.svg'
import userImg from '/user.svg'

function ArrowDown() {
  return (
    <svg
      xmlns='http://www.w3.org/2000/svg'
      width='16'
      height='16'
      fill='currentColor'
      className='bi bi-arrow-down'
      viewBox='0 0 16 16'
    >
      <path
        fillRule='evenodd'
        d='M8 1a.5.5 0 0 1 .5.5v11.793l3.146-3.147a.5.5 0 0 1 .708.708l-4 4a.5.5 0 0 1-.708 0l-4-4a.5.5 0 0 1 .708-.708L7.5 13.293V1.5A.5.5 0 0 1 8 1'
      />
    </svg>
  )
}

export function ArrowUp() {
  return (
    <svg
      xmlns='http://www.w3.org/2000/svg'
      width='16'
      height='16'
      fill='currentColor'
      className='bi bi-arrow-up'
      viewBox='0 0 16 16'
    >
      <path
        fillRule='evenodd'
        d='M8 15a.5.5 0 0 0 .5-.5V2.707l3.146 3.147a.5.5 0 0 0 .708-.708l-4-4a.5.5 0 0 0-.708 0l-4 4a.5.5 0 1 0 .708.708L7.5 2.707V14.5a.5.5 0 0 0 .5.5'
      />
    </svg>
  )
}

export function Copy() {
  return (
    <svg
      xmlns='http://www.w3.org/2000/svg'
      width='16'
      height='16'
      fill='currentColor'
      className='bi bi-copy'
      viewBox='0 0 16 16'
    >
      <path
        fillRule='evenodd'
        d='M4 2a2 2 0 0 1 2-2h8a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2zm2-1a1 1 0 0 0-1 1v8a1 1 0 0 0 1 1h8a1 1 0 0 0 1-1V2a1 1 0 0 0-1-1zM2 5a1 1 0 0 0-1 1v8a1 1 0 0 0 1 1h8a1 1 0 0 0 1-1v-1h1v1a2 2 0 0 1-2 2H2a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h1v1z'
      />
    </svg>
  )
}

export function MessageLoading() {
  return (
    <svg width='24' height='24' viewBox='0 0 24 24' xmlns='http://www.w3.org/2000/svg' className='text-foreground'>
      <circle cx='4' cy='12' r='2' fill='currentColor'>
        <animate
          id='spinner_qFRN'
          begin='0;spinner_OcgL.end+0.25s'
          attributeName='cy'
          calcMode='spline'
          dur='0.6s'
          values='12;6;12'
          keySplines='.33,.66,.66,1;.33,0,.66,.33'
        />
      </circle>
      <circle cx='12' cy='12' r='2' fill='currentColor'>
        <animate
          begin='spinner_qFRN.begin+0.1s'
          attributeName='cy'
          calcMode='spline'
          dur='0.6s'
          values='12;6;12'
          keySplines='.33,.66,.66,1;.33,0,.66,.33'
        />
      </circle>
      <circle cx='20' cy='12' r='2' fill='currentColor'>
        <animate
          id='spinner_OcgL'
          begin='spinner_qFRN.begin+0.2s'
          attributeName='cy'
          calcMode='spline'
          dur='0.6s'
          values='12;6;12'
          keySplines='.33,.66,.66,1;.33,0,.66,.33'
        />
      </circle>
    </svg>
  )
}

const btnVariant = {
  variants: {
    variant: {
      default: 'bg-primary text-primary-foreground shadow-xs hover:bg-primary/90',
      destructive: 'bg-destructive text-white shadow-xs hover:bg-destructive/90 focus-visible:ring-destructive/20',
      outline: 'border bg-background shadow-xs hover:bg-accent hover:text-accent-foreground',
      secondary: 'bg-secondary text-secondary-foreground shadow-xs hover:bg-secondary/80',
      ghost: 'hover:bg-accent hover:text-accent-foreground'
    },
    size: {
      default: 'h-9 px-4 py-2 has-[>svg]:px-3',
      sm: 'h-8 rounded-md gap-1.5 px-3 has-[>svg]:px-2.5',
      lg: 'h-10 rounded-md px-6 has-[>svg]:px-4',
      xl: 'h-16 rounded-md px-8 has-[>svg]:px-6 text-xl',
      icon: 'size-9'
    }
  },
  defaultVariants: {
    variant: 'default',
    size: 'default'
  }
}
const chatBubbleVariant = {
  variants: {
    variant: {
      received: 'self-start',
      sent: 'self-end flex-row-reverse'
    }
  },
  defaultVariants: {
    variant: 'received'
  }
}
const chatBubbleMessageVariants = {
  variants: {
    variant: {
      received: 'bg-secondary text-secondary-foreground rounded-r-lg rounded-tl-lg',
      sent: 'bg-primary text-primary-foreground rounded-l-lg rounded-tr-lg'
    }
  },
  defaultVariants: {
    variant: 'received'
  }
}

export function Textarea({ className, ...props }) {
  return <textarea data-slot='textarea' className={`${className}`} {...props} />
}

export function Button({ className, variant, size, ...props }) {
  const selVariant = variant ?? btnVariant.defaultVariants.variant
  const selSize = size ?? btnVariant.defaultVariants.size
  const clsVariant = btnVariant.variants.variant[selVariant]
  const clsSize = btnVariant.variants.size[selSize]
  return (
    <button
      className={`${clsVariant} ${clsSize} ${className} inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-all disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 shrink-0 [&_svg]:shrink-0 outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 aria-invalid:border-destructive`}
      {...props}
    />
  )
}

export function Avatar({ variant, ...props }) {
  const img = variant === 'sent' ? userImg : botImg
  return (
    <div
      className='relative flex h-8 w-8 shrink-0 overflow-hidden rounded-full'
      style={{ backgroundImage: `url(${img})`, backgroundSize: 'cover' }}
      {...props}
    ></div>
  )
}

export function ChatBubble({ variant, children, ...props }) {
  const cls = chatBubbleVariant.variants.variant[variant]
  return (
    <div className='w-full' {...props}>
      <div className='flex flex-col md:max-w-4xl mx-auto group'>
        <div className={`flex gap-2 max-w-[60%] relative items-end ${cls}`}>
          {React.Children.map(children, child =>
            React.isValidElement(child) && typeof child.type !== 'string'
              ? React.cloneElement(child, {
                  variant
                })
              : child
          )}
        </div>
      </div>
    </div>
  )
}

export function ChatBubbleMessage({ className, variant, isLoading = false, children, ...props }) {
  const selVariant = variant ?? chatBubbleMessageVariants.defaultVariants.variant
  const clsVariant = chatBubbleMessageVariants.variants.variant[selVariant]
  return (
    <div className={`p-4 break-words max-w-full whitespace-pre-wrap ${clsVariant} ${className}`} {...props}>
      {isLoading ? (
        <div className='flex items-center space-x-2'>
          <MessageLoading />
        </div>
      ) : (
        children
      )}
    </div>
  )
}

export function ChatBubbleAction({ icon, onClick, className, variant = 'ghost', size = 'icon', ...props }) {
  return (
    <Button variant={variant} size={size} className={className} onClick={onClick} {...props}>
      {icon}
    </Button>
  )
}

export function ChatBubbleActionWrapper({ variant, className, children, ...props }) {
  const variantCls = variant === 'sent' ? '-left-1 -translate-x-full flex-row-reverse' : '-right-1 translate-x-full'
  return (
    <div
      className={`absolute top-1/2 -translate-y-1/2 flex opacity-0 group-hover:opacity-100 transition-opacity duration-200 ${variantCls} ${className}`}
      {...props}
    >
      {children}
    </div>
  )
}

export function ChatInput({ className, ...props }) {
  const handleInput = e => {
    const textarea = e.target
    textarea.style.height = 'auto'
    textarea.style.height = `${textarea.scrollHeight}px`
  }
  return (
    <Textarea
      autoComplete='off'
      name='message'
      className={`min-h-12 max-h-[25dvh] px-4 py-3 bg-background text-sm placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50 w-full flex items-center h-16 resize-none rounded-lg border-0 shadow-none focus-visible:ring-0 ${className}`}
      onInput={handleInput}
      {...props}
    />
  )
}
