import { createLogger, transports, format } from 'winston';

export class SeaLogger {
  private PREFIX = '[Seaplane] ';
  private transportConsole = new transports.Console({
    level: SeaLogger.WARNING,
  });
  private logger = createLogger({
    transports: [this.transportConsole],
    format: format.combine(
      format.label({
        label: this.PREFIX,
      }),
      format.timestamp({
        format: 'MMM-DD-YYYY HH:mm:ss',
      }),
      format.printf((info) => `${info.level}: ${info.label}: ${[info.timestamp]}: ${info.message}`),
    ),
  });

  public static CRITICAL = 'crit';
  public static ERROR = 'error';
  public static WARNING = 'warning';
  public static INFO = 'info';
  public static DEBUG = 'debug';

  debug(message: string) {
    this.logger.debug(message);
  }

  info(message: string) {
    this.logger.info(message);
  }

  error(message: string) {
    this.logger.error(message);
  }

  level(level: string) {
    this.transportConsole.level = level;
  }

  disable() {
    this.transportConsole.level = SeaLogger.CRITICAL;
  }

  enable(level: string = SeaLogger.WARNING) {
    this.transportConsole.level = level;
  }
}

export const log = new SeaLogger();
log.level(SeaLogger.WARNING);
