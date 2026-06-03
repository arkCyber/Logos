<script setup lang="ts">
import { ref, computed } from 'vue';

interface Comment {
  id: string;
  author: string;
  text: string;
  timestamp: number;
  resolved: boolean;
  replies?: Comment[];
}

interface Props {
  show: boolean;
  comments: Comment[];
}

interface Emits {
  (e: 'update:show', value: boolean): void;
  (e: 'add-comment', comment: Omit<Comment, 'id' | 'timestamp'>): void;
  (e: 'resolve-comment', commentId: string): void;
  (e: 'delete-comment', commentId: string): void;
  (e: 'reply-comment', commentId: string, reply: string): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

// New comment text
const newCommentText = ref('');

// Reply text
const replyText = ref<Record<string, string>>({});

// Active comment for reply
const activeReplyId = ref<string | null>(null);

// Filter
const showResolved = ref(false);

// Computed
const filteredComments = computed(() => {
  if (showResolved.value) {
    return props.comments;
  }
  return props.comments.filter(c => !c.resolved);
});

// Format timestamp
const formatTimestamp = (timestamp: number) => {
  const date = new Date(timestamp);
  const now = new Date();
  const diff = now.getTime() - date.getTime();
  const minutes = Math.floor(diff / 60000);
  const hours = Math.floor(diff / 3600000);
  const days = Math.floor(diff / 86400000);

  if (minutes < 1) {
return '刚刚';
}
  if (minutes < 60) {
return `${minutes}分钟前`;
}
  if (hours < 24) {
return `${hours}小时前`;
}
  if (days < 7) {
return `${days}天前`;
}
  return date.toLocaleDateString();
};

// Add comment
const addComment = () => {
  if (!newCommentText.value.trim()) {
return;
}
  
  emit('add-comment', {
    author: '当前用户',
    text: newCommentText.value,
    resolved: false
  });
  
  newCommentText.value = '';
};

// Resolve comment
const resolveComment = (commentId: string) => {
  emit('resolve-comment', commentId);
};

// Delete comment
const deleteComment = (commentId: string) => {
  if (confirm('确定要删除此批注吗？')) {
    emit('delete-comment', commentId);
  }
};

// Start reply
const startReply = (commentId: string) => {
  activeReplyId.value = commentId;
  replyText.value[commentId] = '';
};

// Cancel reply
const cancelReply = () => {
  activeReplyId.value = null;
};

// Submit reply
const submitReply = (commentId: string) => {
  const text = replyText.value[commentId];
  if (!text?.trim()) {
return;
}
  
  emit('reply-comment', commentId, text);
  replyText.value[commentId] = '';
  activeReplyId.value = null;
};

// Close panel
const closePanel = () => {
  emit('update:show', false);
};
</script>

<template>
  <Transition name="slide">
    <div v-if="show" class="comments-panel editor-side-panel editor-side-panel--right">
      <!-- Header -->
      <div class="panel-header">
        <h3 class="panel-title">批注</h3>
        <div class="panel-actions">
          <label class="checkbox-label">
            <input v-model="showResolved" type="checkbox" />
            <span>显示已解决</span>
          </label>
          <button class="close-btn" type="button" @click="closePanel">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="20"
              height="20"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
            >
              <line x1="18" y1="6" x2="6" y2="18" />
              <line x1="6" y1="6" x2="18" y2="18" />
            </svg>
          </button>
        </div>
      </div>

      <!-- Comments list -->
      <div class="comments-list">
        <div
          v-for="comment in filteredComments"
          :key="comment.id"
          class="comment-item"
          :class="{ resolved: comment.resolved }"
        >
          <!-- Comment header -->
          <div class="comment-header">
            <div class="comment-author">{{ comment.author }}</div>
            <div class="comment-time">{{ formatTimestamp(comment.timestamp) }}</div>
            <div class="comment-actions">
              <button
                v-if="!comment.resolved"
                class="action-btn"
                title="解决"
                type="button"
                @click="resolveComment(comment.id)"
              >
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  width="16"
                  height="16"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                >
                  <polyline points="20 6 9 17 4 12" />
                </svg>
              </button>
              <button
                class="action-btn"
                title="删除"
                type="button"
                @click="deleteComment(comment.id)"
              >
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  width="16"
                  height="16"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                >
                  <polyline points="3 6 5 6 21 6" />
                  <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" />
                </svg>
              </button>
            </div>
          </div>

          <!-- Comment content -->
          <div class="comment-content">{{ comment.text }}</div>

          <!-- Replies -->
          <div v-if="comment.replies && comment.replies.length > 0" class="comment-replies">
            <div
              v-for="reply in comment.replies"
              :key="reply.id"
              class="reply-item"
            >
              <div class="reply-header">
                <span class="reply-author">{{ reply.author }}</span>
                <span class="reply-time">{{ formatTimestamp(reply.timestamp) }}</span>
              </div>
              <div class="reply-content">{{ reply.text }}</div>
            </div>
          </div>

          <!-- Reply input -->
          <div v-if="activeReplyId === comment.id" class="reply-input-container">
            <textarea
              v-model="replyText[comment.id]"
              placeholder="输入回复..."
              class="reply-textarea"
            ></textarea>
            <div class="reply-actions">
              <button class="btn secondary" type="button" @click="cancelReply">
                取消
              </button>
              <button class="btn primary" type="button" @click="submitReply(comment.id)">
                回复
              </button>
            </div>
          </div>

          <!-- Reply button -->
          <button
            v-if="activeReplyId !== comment.id"
            class="reply-btn"
            type="button"
            @click="startReply(comment.id)"
          >
            回复
          </button>
        </div>

        <!-- Empty state -->
        <div v-if="filteredComments.length === 0" class="empty-state">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="48"
            height="48"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="1.5"
          >
            <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" />
          </svg>
          <p>暂无批注</p>
        </div>
      </div>

      <!-- New comment input -->
      <div class="new-comment">
        <textarea
          v-model="newCommentText"
          placeholder="添加新批注..."
          class="new-comment-textarea"
        ></textarea>
        <button
          class="add-comment-btn"
          :disabled="!newCommentText.trim()"
          type="button"
          @click="addComment"
        >
          添加批注
        </button>
      </div>
    </div>
  </Transition>
</template>

<style scoped>
.comments-panel {
  width: var(--editor-sidebar-comments-width, 350px);
  min-width: var(--editor-sidebar-comments-width, 350px);
  max-width: var(--editor-sidebar-comments-width, 350px);
  height: 100%;
  background: var(--word-bg-page);
  display: flex;
  flex-direction: column;
  flex-shrink: 0;
  overflow: hidden;
}

/* Header */
.panel-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px;
  border-bottom: 1px solid var(--word-border);
}

.panel-title {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: var(--word-text-primary);
}

.panel-actions {
  display: flex;
  align-items: center;
  gap: 12px;
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 13px;
  color: var(--word-text-secondary);
  cursor: pointer;
}

.close-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  background: transparent;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  color: var(--word-text-secondary);
  transition: all 0.15s ease;
}

.close-btn:hover {
  background: var(--word-button-hover);
  color: var(--word-text-primary);
}

/* Comments list */
.comments-list {
  flex: 1;
  overflow-y: auto;
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.comment-item {
  padding: 12px;
  background: var(--word-button-bg);
  border: 1px solid var(--word-button-border);
  border-radius: 6px;
  transition: all 0.15s ease;
}

.comment-item.resolved {
  opacity: 0.6;
}

.comment-item:hover {
  border-color: var(--word-button-border-hover);
}

.comment-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 8px;
}

.comment-author {
  font-size: 13px;
  font-weight: 600;
  color: var(--word-text-primary);
}

.comment-time {
  font-size: 12px;
  color: var(--word-text-secondary);
}

.comment-actions {
  display: flex;
  gap: 4px;
}

.action-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  background: transparent;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  color: var(--word-text-secondary);
  transition: all 0.15s ease;
}

.action-btn:hover {
  background: var(--word-button-hover);
  color: var(--word-text-primary);
}

.comment-content {
  font-size: 14px;
  color: var(--word-text-primary);
  line-height: 1.5;
  margin-bottom: 8px;
}

/* Replies */
.comment-replies {
  margin-top: 8px;
  padding-left: 12px;
  border-left: 2px solid var(--word-button-border);
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.reply-item {
  padding: 8px;
  background: var(--word-bg-page);
  border-radius: 4px;
}

.reply-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 4px;
}

.reply-author {
  font-size: 12px;
  font-weight: 600;
  color: var(--word-text-primary);
}

.reply-time {
  font-size: 11px;
  color: var(--word-text-secondary);
}

.reply-content {
  font-size: 13px;
  color: var(--word-text-primary);
  line-height: 1.4;
}

/* Reply input */
.reply-input-container {
  margin-top: 8px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.reply-textarea {
  width: 100%;
  min-height: 60px;
  padding: 8px;
  border: 1px solid var(--word-button-border);
  border-radius: 4px;
  background: var(--word-bg-page);
  color: var(--word-text-primary);
  font-size: 13px;
  font-family: var(--word-font-ui);
  resize: vertical;
}

.reply-textarea:focus {
  outline: none;
  border-color: var(--word-button-border-hover);
}

.reply-actions {
  display: flex;
  gap: 8px;
  justify-content: flex-end;
}

.reply-btn {
  margin-top: 8px;
  padding: 6px 12px;
  background: transparent;
  border: 1px solid var(--word-button-border);
  border-radius: 4px;
  cursor: pointer;
  font-size: 12px;
  color: var(--word-text-secondary);
  transition: all 0.15s ease;
}

.reply-btn:hover {
  background: var(--word-button-hover);
  color: var(--word-text-primary);
}

/* Empty state */
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 12px;
  padding: 40px 20px;
  color: var(--word-text-secondary);
}

.empty-state p {
  margin: 0;
  font-size: 14px;
}

/* New comment */
.new-comment {
  padding: 16px;
  border-top: 1px solid var(--word-border);
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.new-comment-textarea {
  width: 100%;
  min-height: 80px;
  padding: 12px;
  border: 1px solid var(--word-button-border);
  border-radius: 4px;
  background: var(--word-button-bg);
  color: var(--word-text-primary);
  font-size: 14px;
  font-family: var(--word-font-ui);
  resize: vertical;
}

.new-comment-textarea:focus {
  outline: none;
  border-color: var(--word-button-border-hover);
}

.add-comment-btn {
  padding: 8px 16px;
  background: var(--word-button-active);
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  color: var(--word-text-primary);
  transition: all 0.15s ease;
  align-self: flex-end;
}

.add-comment-btn:hover:not(:disabled) {
  background: var(--word-button-pressed);
}

.add-comment-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn {
  padding: 6px 12px;
  border-radius: 4px;
  font-size: 13px;
  cursor: pointer;
  transition: all 0.15s ease;
  border: none;
}

.btn.primary {
  background: var(--word-button-active);
  color: var(--word-text-primary);
}

.btn.primary:hover {
  background: var(--word-button-pressed);
}

.btn.secondary {
  background: var(--word-button-bg);
  border: 1px solid var(--word-button-border);
  color: var(--word-text-primary);
}

.btn.secondary:hover {
  background: var(--word-button-hover);
  border-color: var(--word-button-border-hover);
}

/* Transition */
.slide-enter-active,
.slide-leave-active {
  transition: opacity 0.2s ease;
}

.slide-enter-from,
.slide-leave-to {
  opacity: 0;
}

/* Dark mode */
:global(.dark) .comments-panel {
  background: var(--word-bg-canvas);
  box-shadow: -4px 0 12px rgba(0, 0, 0, 0.4);
}
</style>
