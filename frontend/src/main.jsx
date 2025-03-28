import React, { useState, useRef, useEffect } from 'react'
import ReactDOM from 'react-dom/client'
import '/index.css'
import { backend } from 'declarations/backend'
import {
  ArrowUp,
  Avatar,
  Button,
  ChatBubble,
  ChatBubbleMessage,
  ChatBubbleActionWrapper,
  ChatInput,
  Copy,
  ChatBubbleAction
} from './components'

const ChatActions = [
  {
    icon: Copy,
    type: 'copy'
  }
]

const App = () => {
  const [chat, setChat] = useState([
    { role: { assistant: null }, content: 'hello world' },
    {
      role: { user: null },
      content:
        'lorem ipsum lakjf asokdf jaosdif jaosdf joaskdjf oa jfoiasjo ajsodi jfaiosd jfoasid jfasodif jaosidj\n\nafjsdljf'
    }
  ])
  const [inputValue, setInputValue] = useState('')
  const [isLoading, setIsLoading] = useState(false)
  const [isStarted, setIsStarted] = useState(false)
  const chatBoxRef = useRef(null)

  const formatDate = date => {
    const h = '0' + date.getHours()
    const m = '0' + date.getMinutes()
    return `${h.slice(-2)}:${m.slice(-2)}`
  }

  const askAgent = async input => {
    try {
      const response = await backend.prompt(input)
      setChat(prevChat => {
        const newChat = [...prevChat]
        newChat.pop()
        newChat.push({ role: { assistant: null }, content: response })
        return newChat
      })
    } catch (e) {
      console.error(e)
      const eStr = String(e)
      const match = eStr.match(/(SysTransient|CanisterReject), \\+"([^\\"]+)/)
      if (match) {
        alert(match[2])
      }
      setChat(prevChat => {
        const newChat = [...prevChat]
        newChat.pop()
        return newChat
      })
    } finally {
      setIsLoading(false)
    }
  }

  const onKeyDown = e => {
    if (e.key === 'Enter' && !e.shiftKey) {
      handleSubmit(e)
    }
  }

  const handleSubmit = e => {
    e.preventDefault()
    if (!inputValue.trim() || isLoading) return

    const userMessage = {
      role: { user: null },
      content: inputValue
    }
    const thinkingMessage = {
      role: { assistant: null },
      content: 'Thinking',
      loading: true
    }
    setChat(prevChat => [...prevChat, userMessage, thinkingMessage])
    setInputValue('')
    setIsLoading(true)
    askAgent(inputValue)
  }

  const handleCmd = cmd => {
    if (cmd === '/start') {
      askAgent(cmd)
    } else if (cmd === '/about') {
      askAgent(cmd)
    }
  }

  const handleActionClick = async (action, messageIndex) => {
    console.log('Action clicked:', action, 'Message index:', messageIndex)

    if (action === 'copy') {
      const message = chat[messageIndex]
      if (message && message.role === 'assistant') {
        navigator.clipboard.writeText(message.content)
      }
    }
  }

  useEffect(() => {
    if (chatBoxRef.current) {
      chatBoxRef.current.scrollTop = chatBoxRef.current.scrollHeight
    }
  }, [chat])

  return (
    <div className='flex flex-col h-full w-full overflow-hidden'>
      <div className='flex flex-col relative w-full h-full'>
        <div className='sticky'>header</div>
        <div className='flex flex-col w-full h-full p-4 overflow-y-auto'>
          <div className='flex flex-col gap-6'>
            {chat.map((message, index) => {
              const variant = 'user' in message.role ? 'sent' : 'received'
              return (
                <ChatBubble key={index} variant={variant}>
                  <Avatar variant={variant} />
                  <ChatBubbleMessage variant={variant} isLoading={message.loading}>
                    {message.content}
                  </ChatBubbleMessage>
                  <ChatBubbleActionWrapper>
                    {ChatActions.map(({ icon: Icon, type }) => (
                      <ChatBubbleAction
                        className='size-7'
                        key={type}
                        icon={<Icon className='size-4' />}
                        onClick={() => console.log('Action ' + type + ' clicked for message ' + index)}
                      />
                    ))}
                  </ChatBubbleActionWrapper>
                </ChatBubble>
              )
            })}
          </div>
        </div>
        {!isStarted && (
          <div className='w-full px-4 pb-4 relative'>
            <div className='mx-auto flex flex-1 gap-4 md:gap-5 lg:gap-6 md:max-w-3xl lg:max-w-[40rem] xl:max-w-[48rem]'>
              <div className='relative z-[1] flex max-w-full flex-1 flex-col h-full'>
                <form
                  className='relative rounded-lg border bg-muted focus-within:ring-1 focus-within:ring-ring py-3 pl-3'
                  onSubmit={handleSubmit}
                >
                  <ChatInput
                    onKeyDown={onKeyDown}
                    onChange={e => setInputValue(e.target.value)}
                    value={inputValue}
                    placeholder='Write your story'
                    className='bg-transparent'
                  />
                  <div className='flex p-3 pt-0'>
                    <Button type='submit' disabled={isLoading || !inputValue} className='ml-auto gap-1.5' size='sm'>
                      Send
                      <ArrowUp />
                    </Button>
                  </div>
                </form>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  )
}

export default App

ReactDOM.createRoot(document.getElementById('root')).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
)
