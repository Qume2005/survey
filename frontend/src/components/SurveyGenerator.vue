<template>
  <div class="survey-generator">
    <h1>问卷生成器</h1>
    <input
      type="number"
      v-model="researcherId"
      placeholder="输入研究员 ID"
    />
    <button @click="generateSurvey">生成问卷</button>

    <div v-if="survey">
      <h2>生成的问卷:</h2>
      <p>问卷 ID: {{ survey.survey_id }}</p>
      <p>问卷题目列表: {{ survey.survey_list.join(', ') }}</p>
    </div>

    <div v-if="errorMessage" class="error">
      <p>{{ errorMessage }}</p>
    </div>
  </div>
</template>

<script>
import axios from 'axios';

export default {
  data() {
    return {
      researcherId: '',
      survey: null,
      errorMessage: '',
    };
  },
  methods: {
    async generateSurvey() {
      this.errorMessage = ''; // Reset error message
      this.survey = null; // Reset survey data

      // Basic validation
      if (!this.researcherId) {
        this.errorMessage = '请输入有效的研究员 ID.';
        return;
      }

      try {
        const response = await axios.post('/api/generate', {
          researcher_id: this.researcherId,
        });

        this.survey = response.data;
      } catch (error) {
        this.errorMessage = '生成问卷时出错，请重试.';
        console.error(error);
      }
    },
  },
};
</script>

<style scoped>
.survey-generator {
  max-width: 400px;
  margin: 0 auto;
  text-align: center;
}

.error {
  color: red;
}
</style>
