/// <reference path="./global.d.ts" />
// @ts-check

export class TranslationService {
  constructor(api) {
    this.api = api;
  }

  free(text) {
    return this.api.fetch(text).then(result => result.translation);
  }

  batch(texts) {
    if (texts.length === 0) {
      return Promise.reject(new BatchIsEmpty());
    }

    return Promise.all(texts.map(text => this.free(text)));
  }

  request(text) {
    return new Promise((resolve, reject) => {
      let attempts = 0;

      const attempt = () => {
        attempts++;

        this.api.request(text, error => {
          if (!error) {
            resolve();
          } else if (attempts >= 3) {
            reject(error);
          } else {
            attempt();
          }
        });
      };

      attempt();
    });
  }

  premium(text, minimumQuality) {
    return this.api
      .fetch(text)
      .then(result => {
        if (result.quality < minimumQuality) {
          throw new QualityThresholdNotMet(text);
        }
        return result.translation;
      })
      .catch(error => {
        // ✅ CORRECT FIX
        if (error.constructor.name === 'NotAvailable') {
          return this.request(text).then(() =>
            this.premium(text, minimumQuality),
          );
        }

        // ❌ Untranslatable or any other error → STOP
        throw error;
      });
  }
}

export class QualityThresholdNotMet extends Error {
  constructor(text) {
    super(
      `The translation of ${text} does not meet the requested quality threshold.`,
    );
    this.text = text;
  }
}

export class BatchIsEmpty extends Error {
  constructor() {
    super(
      'Requested a batch translation, but there are no texts in the batch.',
    );
  }
}
