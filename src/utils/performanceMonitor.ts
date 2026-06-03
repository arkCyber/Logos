/**
 * 航空航天级性能监控系统
 * 提供全面的性能监控、分析和优化功能
 */

import { logger, LogCategory, LogLevel } from './logger'; // eslint-disable-line @typescript-eslint/no-unused-vars

/**
 * 性能指标接口
 */
export interface PerformanceMetrics {
  timestamp: number;
  memory: MemoryMetrics;
  cpu: CPUMetrics;
  rendering: RenderingMetrics;
  network: NetworkMetrics;
  custom: Record<string, number>;
}

/**
 * 内存指标
 */
export interface MemoryMetrics {
  usedJSHeapSize: number;
  totalJSHeapSize: number;
  jsHeapSizeLimit: number;
  memoryUsagePercentage: number;
}

/**
 * CPU指标
 */
export interface CPUMetrics {
  estimatedUsage: number;
  threadCount: number;
}

/**
 * 渲染指标
 */
export interface RenderingMetrics {
  fps: number;
  frameTime: number;
  droppedFrames: number;
  totalFrames: number;
  layoutShifts: number;
  paintTime: number;
}

/**
 * 网络指标
 */
export interface NetworkMetrics {
  connectionType: string;
  downlink: number;
  rtt: number;
  effectiveType: string;
  requestCount: number;
  averageResponseTime: number;
}

/**
 * 性能警报接口
 */
export interface PerformanceAlert {
  id: string;
  type: 'memory' | 'cpu' | 'rendering' | 'network';
  severity: 'warning' | 'critical';
  message: string;
  value: number;
  threshold: number;
  timestamp: number;
}

/**
 * 性能配置接口
 */
export interface PerformanceConfig {
  enableMemoryMonitoring: boolean;
  enableCPUMonitoring: boolean;
  enableRenderingMonitoring: boolean;
  enableNetworkMonitoring: boolean;
  samplingInterval: number;
  alertThresholds: {
    memoryUsage: number;
    cpuUsage: number;
    fps: number;
    frameTime: number;
  };
  enableAutoOptimization: boolean;
}

/**
 * 默认配置
 */
const DEFAULT_CONFIG: PerformanceConfig = {
  enableMemoryMonitoring: true,
  enableCPUMonitoring: true,
  enableRenderingMonitoring: true,
  enableNetworkMonitoring: true,
  samplingInterval: 1000,
  alertThresholds: {
    memoryUsage: 80, // 80%
    cpuUsage: 80, // 80%
    fps: 30, // 30 FPS
    frameTime: 33 // 33ms
  },
  enableAutoOptimization: false
};

/**
 * 航空航天级性能监控器
 */
export class PerformanceMonitor {
  private config: PerformanceConfig;
  private metrics: PerformanceMetrics[] = [];
  private alerts: PerformanceAlert[] = [];
  private monitoringInterval: number | null = null;
  private frameCount = 0;
  private lastFrameTime = performance.now();
  private droppedFrames = 0;
  private layoutShifts = 0;
  private networkRequests: Map<string, number[]> = new Map();
  private customMetrics: Map<string, number[]> = new Map();

  constructor(config: Partial<PerformanceConfig> = {}) {
    this.config = { ...DEFAULT_CONFIG, ...config };
    this.initialize();
  }

  /**
   * 初始化性能监控
   */
  private initialize(): void {
    logger.info('Performance monitor initialized', { config: this.config }, LogCategory.PERFORMANCE);

    // 设置性能观察者
    this.setupPerformanceObservers();

    // 开始监控
    this.startMonitoring();
  }

  /**
   * 设置性能观察者
   */
  private setupPerformanceObservers(): void {
    if (typeof window === 'undefined' || !window.PerformanceObserver) {
      return;
    }

    try {
      // 观察布局偏移
      const layoutShiftObserver = new PerformanceObserver((list) => {
        for (const entry of list.getEntries()) {
          if (!(entry as any).hadRecentInput) {
            this.layoutShifts++;
          }
        }
      });
      layoutShiftObserver.observe({ entryTypes: ['layout-shift'] });

      // 观察绘制时间
      const paintObserver = new PerformanceObserver((list) => {
        for (const entry of list.getEntries()) {
          logger.debug('Paint entry', { entry }, LogCategory.PERFORMANCE);
        }
      });
      paintObserver.observe({ entryTypes: ['paint'] });

      // 观察资源加载
      const resourceObserver = new PerformanceObserver((list) => {
        for (const entry of list.getEntries()) {
          if (entry.entryType === 'resource') {
            const resource = entry as PerformanceResourceTiming;
            const url = resource.name;
            const duration = resource.duration;

            if (!this.networkRequests.has(url)) {
              this.networkRequests.set(url, []);
            }
            const durations = this.networkRequests.get(url);
            if (durations) {
              durations.push(duration);
            }
          }
        }
      });
      resourceObserver.observe({ entryTypes: ['resource'] });

      logger.info('Performance observers setup complete', {}, LogCategory.PERFORMANCE);
    } catch (error) {
      logger.error('Failed to setup performance observers', error, LogCategory.PERFORMANCE);
    }
  }

  /**
   * 开始监控
   */
  startMonitoring(): void {
    if (this.monitoringInterval) {
      return;
    }

    this.monitoringInterval = window.setInterval(() => {
      this.collectMetrics();
    }, this.config.samplingInterval);

    // 开始帧率监控
    this.startFrameMonitoring();

    logger.info('Performance monitoring started', {}, LogCategory.PERFORMANCE);
  }

  /**
   * 停止监控
   */
  stopMonitoring(): void {
    if (this.monitoringInterval) {
      clearInterval(this.monitoringInterval);
      this.monitoringInterval = null;
    }

    logger.info('Performance monitoring stopped', {}, LogCategory.PERFORMANCE);
  }

  /**
   * 开始帧率监控
   */
  private startFrameMonitoring(): void {
    const measureFrame = () => {
      const now = performance.now();
      const delta = now - this.lastFrameTime;
      this.lastFrameTime = now;

      this.frameCount++;

      // 检测掉帧
      if (delta > 50) { // 超过50ms认为是掉帧
        this.droppedFrames++;
      }

      requestAnimationFrame(measureFrame);
    };

    requestAnimationFrame(measureFrame);
  }

  /**
   * 收集性能指标
   */
  private collectMetrics(): void {
    const metrics: PerformanceMetrics = {
      timestamp: Date.now(),
      memory: this.collectMemoryMetrics(),
      cpu: this.collectCPUMetrics(),
      rendering: this.collectRenderingMetrics(),
      network: this.collectNetworkMetrics(),
      custom: this.collectCustomMetrics()
    };

    this.metrics.push(metrics);

    // 限制指标历史数量
    if (this.metrics.length > 1000) {
      this.metrics.shift();
    }

    // 检查性能警报
    this.checkAlerts(metrics);

    // 自动优化（如果启用）
    if (this.config.enableAutoOptimization) {
      this.autoOptimize(metrics);
    }
  }

  /**
   * 收集内存指标
   */
  private collectMemoryMetrics(): MemoryMetrics {
    if (!this.config.enableMemoryMonitoring) {
      return {
        usedJSHeapSize: 0,
        totalJSHeapSize: 0,
        jsHeapSizeLimit: 0,
        memoryUsagePercentage: 0
      };
    }

    if (typeof performance === 'undefined' || !(performance as any).memory) {
      return {
        usedJSHeapSize: 0,
        totalJSHeapSize: 0,
        jsHeapSizeLimit: 0,
        memoryUsagePercentage: 0
      };
    }

    const memory = (performance as any).memory;
    const memoryUsagePercentage = (memory.usedJSHeapSize / memory.jsHeapSizeLimit) * 100;

    return {
      usedJSHeapSize: memory.usedJSHeapSize,
      totalJSHeapSize: memory.totalJSHeapSize,
      jsHeapSizeLimit: memory.jsHeapSizeLimit,
      memoryUsagePercentage
    };
  }

  /**
   * 收集CPU指标
   */
  private collectCPUMetrics(): CPUMetrics {
    if (!this.config.enableCPUMonitoring) {
      return {
        estimatedUsage: 0,
        threadCount: 0
      };
    }

    // CPU使用率估算（基于任务执行时间）
    const estimatedUsage = this.estimateCPUUsage();
    const threadCount = navigator.hardwareConcurrency || 0;

    return {
      estimatedUsage,
      threadCount
    };
  }

  /**
   * 估算CPU使用率
   */
  private estimateCPUUsage(): number {
    // 这是一个简化的估算，实际应用中可能需要更精确的方法
    const start = performance.now();
    let _workCount = 0;

    // 执行一些计算工作来测量CPU性能
    for (let i = 0; i < 1000000; i++) {
      _workCount += Math.sqrt(i);
    }

    const end = performance.now();
    const duration = end - start;

    // 基于执行时间估算CPU使用率
    return Math.min(100, (duration / 10) * 100);
  }

  /**
   * 收集渲染指标
   */
  private collectRenderingMetrics(): RenderingMetrics {
    if (!this.config.enableRenderingMonitoring) {
      return {
        fps: 0,
        frameTime: 0,
        droppedFrames: 0,
        totalFrames: 0,
        layoutShifts: 0,
        paintTime: 0
      };
    }

    const now = performance.now();
    const frameTime = now - this.lastFrameTime;
    const fps = 1000 / frameTime;

    return {
      fps,
      frameTime,
      droppedFrames: this.droppedFrames,
      totalFrames: this.frameCount,
      layoutShifts: this.layoutShifts,
      paintTime: this.measurePaintTime()
    };
  }

  /**
   * 测量绘制时间
   */
  private measurePaintTime(): number {
    const paintEntries = performance.getEntriesByType('paint');
    if (paintEntries.length === 0) {
return 0;
}

    const lastPaint = paintEntries[paintEntries.length - 1] as PerformancePaintTiming;
    return lastPaint.duration;
  }

  /**
   * 收集网络指标
   */
  private collectNetworkMetrics(): NetworkMetrics {
    if (!this.config.enableNetworkMonitoring) {
      return {
        connectionType: 'unknown',
        downlink: 0,
        rtt: 0,
        effectiveType: 'unknown',
        requestCount: 0,
        averageResponseTime: 0
      };
    }

    const connection = (navigator as any).connection || (navigator as any).mozConnection || (navigator as any).webkitConnection;

    const connectionType = connection ? connection.effectiveType : 'unknown';
    const downlink = connection ? connection.downlink : 0;
    const rtt = connection ? connection.rtt : 0;
    const effectiveType = connection ? connection.effectiveType : 'unknown';

    // 计算平均响应时间
    let totalResponseTime = 0;
    let requestCount = 0;

    for (const durations of this.networkRequests.values()) {
      for (const duration of durations) {
        totalResponseTime += duration;
        requestCount++;
      }
    }

    const averageResponseTime = requestCount > 0 ? totalResponseTime / requestCount : 0;

    return {
      connectionType,
      downlink,
      rtt,
      effectiveType,
      requestCount,
      averageResponseTime
    };
  }

  /**
   * 收集自定义指标
   */
  private collectCustomMetrics(): Record<string, number> {
    const custom: Record<string, number> = {};

    for (const [name, values] of this.customMetrics.entries()) {
      if (values.length > 0) {
        custom[name] = values[values.length - 1];
      }
    }

    return custom;
  }

  /**
   * 检查性能警报
   */
  private checkAlerts(metrics: PerformanceMetrics): void {
    const { alertThresholds } = this.config;

    // 内存使用警报
    if (metrics.memory.memoryUsagePercentage > alertThresholds.memoryUsage) {
      this.createAlert(
        'memory',
        metrics.memory.memoryUsagePercentage > 90 ? 'critical' : 'warning',
        `内存使用率过高: ${metrics.memory.memoryUsagePercentage.toFixed(1)}%`,
        metrics.memory.memoryUsagePercentage,
        alertThresholds.memoryUsage
      );
    }

    // CPU使用警报
    if (metrics.cpu.estimatedUsage > alertThresholds.cpuUsage) {
      this.createAlert(
        'cpu',
        metrics.cpu.estimatedUsage > 90 ? 'critical' : 'warning',
        `CPU使用率过高: ${metrics.cpu.estimatedUsage.toFixed(1)}%`,
        metrics.cpu.estimatedUsage,
        alertThresholds.cpuUsage
      );
    }

    // 帧率警报
    if (metrics.rendering.fps < alertThresholds.fps) {
      this.createAlert(
        'rendering',
        metrics.rendering.fps < 20 ? 'critical' : 'warning',
        `帧率过低: ${metrics.rendering.fps.toFixed(1)} FPS`,
        metrics.rendering.fps,
        alertThresholds.fps
      );
    }

    // 帧时间警报
    if (metrics.rendering.frameTime > alertThresholds.frameTime) {
      this.createAlert(
        'rendering',
        metrics.rendering.frameTime > 50 ? 'critical' : 'warning',
        `帧时间过长: ${metrics.rendering.frameTime.toFixed(1)}ms`,
        metrics.rendering.frameTime,
        alertThresholds.frameTime
      );
    }
  }

  /**
   * 创建性能警报
   */
  private createAlert(
    type: 'memory' | 'cpu' | 'rendering' | 'network',
    severity: 'warning' | 'critical',
    message: string,
    value: number,
    threshold: number
  ): void {
    const alert: PerformanceAlert = {
      id: `${type}-${Date.now()}`,
      type,
      severity,
      message,
      value,
      threshold,
      timestamp: Date.now()
    };

    this.alerts.push(alert);

    // 限制警报历史数量
    if (this.alerts.length > 100) {
      this.alerts.shift();
    }

    // 记录日志
    if (severity === 'critical') {
      logger.critical(message, { alert }, LogCategory.PERFORMANCE);
    } else {
      logger.warn(message, { alert }, LogCategory.PERFORMANCE);
    }
  }

  /**
   * 自动优化
   */
  private autoOptimize(metrics: PerformanceMetrics): void {
    // 内存优化
    if (metrics.memory.memoryUsagePercentage > 90) {
      this.optimizeMemory();
    }

    // 渲染优化
    if (metrics.rendering.fps < 30) {
      this.optimizeRendering();
    }
  }

  /**
   * 内存优化
   */
  private optimizeMemory(): void {
    logger.info('Executing memory optimization', {}, LogCategory.PERFORMANCE);

    // 清理旧的指标
    if (this.metrics.length > 100) {
      this.metrics = this.metrics.slice(-100);
    }

    // 清理旧的警报
    if (this.alerts.length > 50) {
      this.alerts = this.alerts.slice(-50);
    }

    // 清理网络请求历史
    if (this.networkRequests.size > 100) {
      const keys = Array.from(this.networkRequests.keys());
      keys.slice(0, keys.length - 100).forEach(key => {
        this.networkRequests.delete(key);
      });
    }

    // 触发垃圾回收（如果可用）
    if (typeof (window as any).gc === 'function') {
      (window as any).gc();
    }
  }

  /**
   * 渲染优化
   */
  private optimizeRendering(): void {
    logger.info('Executing rendering optimization', {}, LogCategory.PERFORMANCE);

    // 减少布局偏移检测频率
    // 在实际应用中，这里可以：
    // 1. 减少DOM操作
    // 2. 使用CSS transforms代替top/left
    // 3. 实现虚拟滚动
    // 4. 减少重绘和回流
  }

  /**
   * 记录自定义指标
   */
  recordCustomMetric(name: string, value: number): void {
    if (!this.customMetrics.has(name)) {
      this.customMetrics.set(name, []);
    }

    const values = this.customMetrics.get(name);
    if (values) {
      values.push(value);

      // 限制历史数量
      if (values.length > 100) {
        values.shift();
      }
    }
  }

  /**
   * 获取性能指标
   */
  getMetrics(): PerformanceMetrics[] {
    return [...this.metrics];
  }

  /**
   * 获取最新指标
   */
  getLatestMetrics(): PerformanceMetrics | null {
    return this.metrics.length > 0 ? this.metrics[this.metrics.length - 1] : null;
  }

  /**
   * 获取性能警报
   */
  getAlerts(): PerformanceAlert[] {
    return [...this.alerts];
  }

  /**
   * 获取性能统计
   */
  getPerformanceStatistics(): {
    averageFPS: number;
    averageFrameTime: number;
    averageMemoryUsage: number;
    averageCPUUsage: number;
    totalDroppedFrames: number;
    totalLayoutShifts: number;
  } {
    if (this.metrics.length === 0) {
      return {
        averageFPS: 0,
        averageFrameTime: 0,
        averageMemoryUsage: 0,
        averageCPUUsage: 0,
        totalDroppedFrames: 0,
        totalLayoutShifts: 0
      };
    }

    const totalFPS = this.metrics.reduce((sum, m) => sum + m.rendering.fps, 0);
    const totalFrameTime = this.metrics.reduce((sum, m) => sum + m.rendering.frameTime, 0);
    const totalMemoryUsage = this.metrics.reduce((sum, m) => sum + m.memory.memoryUsagePercentage, 0);
    const totalCPUUsage = this.metrics.reduce((sum, m) => sum + m.cpu.estimatedUsage, 0);

    return {
      averageFPS: totalFPS / this.metrics.length,
      averageFrameTime: totalFrameTime / this.metrics.length,
      averageMemoryUsage: totalMemoryUsage / this.metrics.length,
      averageCPUUsage: totalCPUUsage / this.metrics.length,
      totalDroppedFrames: this.droppedFrames,
      totalLayoutShifts: this.layoutShifts
    };
  }

  /**
   * 导出性能报告
   */
  exportPerformanceReport(): string {
    const statistics = this.getPerformanceStatistics();
    const latestMetrics = this.getLatestMetrics();
    const recentAlerts = this.alerts.slice(-10);

    return JSON.stringify({
      timestamp: Date.now(),
      statistics,
      latestMetrics,
      recentAlerts,
      config: this.config
    }, null, 2);
  }

  /**
   * 下载性能报告
   */
  downloadPerformanceReport(): void {
    const content = this.exportPerformanceReport();
    const blob = new Blob([content], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `performance-report-${Date.now()}.json`;
    a.click();
    URL.revokeObjectURL(url);
  }

  /**
   * 清除性能数据
   */
  clearMetrics(): void {
    this.metrics = [];
    this.alerts = [];
    this.frameCount = 0;
    this.droppedFrames = 0;
    this.layoutShifts = 0;
    this.networkRequests.clear();
    this.customMetrics.clear();

    logger.info('Performance metrics cleared', {}, LogCategory.PERFORMANCE);
  }

  /**
   * 更新配置
   */
  updateConfig(config: Partial<PerformanceConfig>): void {
    this.config = { ...this.config, ...config };
    logger.info('Performance monitor configuration updated', { config: this.config }, LogCategory.PERFORMANCE);

    // 重启监控（如果配置改变）
    if (this.monitoringInterval) {
      this.stopMonitoring();
      this.startMonitoring();
    }
  }
}

// 导出单例
export const performanceMonitor = new PerformanceMonitor();

// 导出便捷函数
export const perf = {
  startMonitoring: () => performanceMonitor.startMonitoring(),
  stopMonitoring: () => performanceMonitor.stopMonitoring(),
  recordMetric: (name: string, value: number) => performanceMonitor.recordCustomMetric(name, value),
  getMetrics: () => performanceMonitor.getMetrics(),
  getLatest: () => performanceMonitor.getLatestMetrics(),
  getStatistics: () => performanceMonitor.getPerformanceStatistics(),
  getAlerts: () => performanceMonitor.getAlerts(),
  exportReport: () => performanceMonitor.exportPerformanceReport(),
  downloadReport: () => performanceMonitor.downloadPerformanceReport(),
  clearMetrics: () => performanceMonitor.clearMetrics(),
  updateConfig: (config: Partial<PerformanceConfig>) => performanceMonitor.updateConfig(config)
};
