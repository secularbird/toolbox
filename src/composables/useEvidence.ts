/**
 * useEvidence Composable
 * Handles evidence/attachment operations for reminders
 */

import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { convertFileSrc } from '@tauri-apps/api/core';
import type { Evidence } from '../types/reminder';

export function useEvidence() {
  const evidenceList = ref<Evidence[]>([]);
  const uploadingFile = ref(false);
  const previewImage = ref<string | null>(null);
  const message = ref("");

  // Load evidence for a specific reminder
  async function loadReminderEvidence(reminderId: number) {
    try {
      evidenceList.value = await invoke("get_reminder_evidence", { reminderId });
    } catch (error) {
      console.error("Failed to load evidence:", error);
    }
  }

  // Handle file upload
  async function handleFileUpload(
    file: File,
    reminderId: number
  ): Promise<Evidence | null> {
    uploadingFile.value = true;
    
    try {
      // Read file as array buffer
      const arrayBuffer = await file.arrayBuffer();
      const fileData = Array.from(new Uint8Array(arrayBuffer));
      
      // Save file to app data directory
      const savedPath = await invoke<string>("save_uploaded_file", {
        fileName: file.name,
        fileData,
      });
      
      // Get mime type
      const mimeType = await invoke<string>("get_mime_type", { filePath: savedPath });
      
      // Determine file type
      let fileType = "document";
      if (mimeType.startsWith("image/")) fileType = "image";
      else if (mimeType.startsWith("video/")) fileType = "video";
      else if (mimeType.startsWith("audio/")) fileType = "audio";
      
      // Add evidence to database
      const evidence = await invoke<Evidence>("add_evidence_to_reminder", {
        reminderId,
        fileType,
        filePath: savedPath,
        fileName: file.name,
        fileSize: file.size,
        mimeType,
        thumbnailPath: null,
        description: null,
        metadata: null,
      });
      
      evidenceList.value.push(evidence);
      message.value = "File attached successfully!";
      setTimeout(() => { message.value = ""; }, 2000);
      
      return evidence;
    } catch (error) {
      message.value = `Error uploading file: ${error}`;
      return null;
    } finally {
      uploadingFile.value = false;
    }
  }

  // Delete evidence item
  async function deleteEvidence(evidenceId: number) {
    try {
      await invoke("delete_evidence_item", { evidenceId });
      evidenceList.value = evidenceList.value.filter(e => e.id !== evidenceId);
      message.value = "Attachment deleted";
      setTimeout(() => { message.value = ""; }, 2000);
      return true;
    } catch (error) {
      message.value = `Error: ${error}`;
      return false;
    }
  }

  // Open evidence file
  async function openEvidence(filePath: string) {
    try {
      await invoke("open_evidence_file", { filePath });
    } catch (error) {
      message.value = `Error opening file: ${error}`;
    }
  }

  // Format file size
  function formatFileSize(bytes: number): string {
    const units = ['B', 'KB', 'MB', 'GB'];
    let size = bytes;
    let unitIndex = 0;
    
    while (size >= 1024 && unitIndex < units.length - 1) {
      size /= 1024;
      unitIndex++;
    }
    
    return `${size.toFixed(2)} ${units[unitIndex]}`;
  }

  // Get file icon based on type
  function getFileIcon(fileType: string, mimeType: string): string {
    if (fileType === "image") return "🖼️";
    if (fileType === "video") return "🎥";
    if (fileType === "audio") return "🎵";
    if (mimeType.includes("pdf")) return "📄";
    if (mimeType.includes("word")) return "📝";
    if (mimeType.includes("excel") || mimeType.includes("spreadsheet")) return "📊";
    if (mimeType.includes("zip") || mimeType.includes("archive")) return "📦";
    return "📎";
  }

  // Show image preview
  function showImagePreview(filePath: string) {
    previewImage.value = convertFileSrc(filePath);
  }

  // Close image preview
  function closeImagePreview() {
    previewImage.value = null;
  }

  // Clear evidence list
  function clearEvidence() {
    evidenceList.value = [];
    previewImage.value = null;
  }

  return {
    evidenceList,
    uploadingFile,
    previewImage,
    message,
    loadReminderEvidence,
    handleFileUpload,
    deleteEvidence,
    openEvidence,
    formatFileSize,
    getFileIcon,
    showImagePreview,
    closeImagePreview,
    clearEvidence,
  };
}
