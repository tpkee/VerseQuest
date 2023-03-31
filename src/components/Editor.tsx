'use client'

import { useEditor, EditorContent } from '@tiptap/react'
import StarterKit from '@tiptap/starter-kit'

interface Editor {
  value: string, 
  onChange?: (content: any) => void,
  isEditable?: boolean
}

export default function Tiptap ({ value, isEditable = true, onChange }: Editor) {
  const editor = useEditor({
    extensions: [
      StarterKit
    ],
    content: value,
    editable: isEditable,
    onUpdate: ({ editor }) => {
      onChange?.(editor.getHTML())
    },
    editorProps: {
      attributes: {
        class: 'border-2 border-slate-100 prose dark:prose-invert prose-sm m-5 focus:outline-none'
      }
    }
  })

  return (
    <EditorContent editor={editor} className='' />
  )
}